#!/usr/bin/env node
/**
 * analyze-crate.mjs — 分析 aacode 本地 crate 迁移到 gpui_learn 需要哪些操作。
 *
 * 用法:
 *   node tools/analyze-crate.mjs packages/paths
 *   node tools/analyze-crate.mjs packages/paths packages/telemetry packages/askpass
 *   node tools/analyze-crate.mjs packages/paths --gpui-learn /path/to/gpui_learn
 *
 * 输出:
 *   1. gpui_learn workspace.dependencies 缺失的条目（直接 copy 粘贴）
 *   2. aacode 侧 Cargo.toml 修改建议（删除 path、加 git 依赖、删 members）
 *   3. 源码同步（replace crate::old_name → new_name）
 *   4. gpui_learn members 缺失检查
 */

import { readFileSync, existsSync, readdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { execSync } from 'node:child_process';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, '..');

// ── TOML 简易解析：只支持我们关心的字段 ──────────────────────────────────────
function parseTOML(text) {
  const result = {};
  let current = result;
  const stack = [{ obj: result, key: null }];
  const reSection = /^\[([^\]]+)\]\s*$/;
  const reValue = /^([A-Za-z_-][\w.-]*)\s*=\s*(.+)$/;

  const lines = text.split('\n');
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    if (!line || line.startsWith('#')) continue;

    const secMatch = line.match(reSection);
    if (secMatch) {
      const name = secMatch[1].trim();
      const keys = name.split('.');
      let obj = result;
      for (const k of keys) {
        if (!(k in obj) || typeof obj[k] !== 'object') obj[k] = {};
        obj = obj[k];
      }
      stack.push({ obj, key: name });
      current = obj;
      continue;
    }

    const valMatch = line.match(reValue);
    if (valMatch) {
      const key = valMatch[1];
      let raw = valMatch[2];

      // 点语法: foo.workspace = true → { workspace: true }
      // foo = "bar" → { workspace: true, ... } 不会出现，简单处理
      if (key.includes('.')) {
        const parts = key.split('.');
        let obj = current;
        for (const p of parts.slice(0, -1)) {
          if (!(p in obj) || typeof obj[p] !== 'object') obj[p] = {};
          obj = obj[p];
        }
        obj[parts[parts.length - 1]] = parseValue(raw);
        current[key.split('.')[0]] = obj; // 方便后续遍历
        continue;
      }

      // 单行内联 table / array 可能跨多行，简单处理到同段结束
      while (
        (raw.includes('{') && !raw.includes('}')) ||
        (raw.includes('[') && !raw.includes(']'))
      ) {
        i++;
        if (i >= lines.length) break;
        raw += ' ' + lines[i].trim();
      }

      current[key] = parseValue(raw);
    }
  }
  return result;
}

function parseValue(raw) {
  raw = raw.trim();
  if (raw === 'true') return true;
  if (raw === 'false') return false;
  if (/^-?\d+(\.\d+)?$/.test(raw)) return raw.includes('.') ? Number(raw) : Number(raw);
  if (raw.startsWith('"') && raw.endsWith('"')) return raw.slice(1, -1);
  if (raw.startsWith('{') && raw.endsWith('}')) return parseInlineTable(raw);
  if (raw.startsWith('[') && raw.endsWith(']')) return parseInlineArray(raw);
  return raw;
}

function parseInlineTable(raw) {
  const obj = {};
  const inner = raw.slice(1, -1);
  const parts = splitTopLevel(inner, ',');
  for (const p of parts) {
    const eq = p.indexOf('=');
    if (eq < 0) continue;
    const key = p.slice(0, eq).trim();
    const val = parseValue(p.slice(eq + 1).trim());
    obj[key] = val;
  }
  return obj;
}

function parseInlineArray(raw) {
  const inner = raw.slice(1, -1);
  if (!inner.trim()) return [];
  const items = splitTopLevel(inner, ',');
  return items.map((s) => parseValue(s.trim()));
}

function splitTopLevel(str, sep) {
  const result = [];
  let depth = 0;
  let buf = '';
  let inStr = false;
  for (let i = 0; i < str.length; i++) {
    const c = str[i];
    if (c === '"') inStr = !inStr;
    if (!inStr) {
      if (c === '{' || c === '[' || c === '(') depth++;
      if (c === '}' || c === ']' || c === ')') depth--;
      if (c === sep && depth === 0) {
        result.push(buf);
        buf = '';
        continue;
      }
    }
    buf += c;
  }
  if (buf) result.push(buf);
  return result;
}

// ── 主逻辑 ──────────────────────────────────────────────────────────────────

function walkDeps(crateToml) {
  /** @type {Array<{section:string, key:string, value:any}>} */
  const all = [];
  const pkg = crateToml.package || {};

  const top = ['dependencies', 'dev-dependencies', 'build-dependencies'];
  for (const sec of top) {
    if (crateToml[sec]) {
      for (const [k, v] of Object.entries(crateToml[sec])) {
        all.push({ section: sec, key: k, value: v });
      }
    }
  }

  for (const [k, v] of Object.entries(crateToml)) {
    if (k.startsWith('target.') && typeof v === 'object') {
      for (const sec of top) {
        if (v[sec]) {
          for (const [dk, dv] of Object.entries(v[sec])) {
            all.push({ section: `${k}.${sec}`, key: dk, value: dv });
          }
        }
      }
    }
  }

  return all;
}

function resolveDepName(entry) {
  // dep key = 写在 Cargo.toml 里的名字（如 `fs` 或 `aa_gpui_kit_fs`）
  // package = 实际 crate 名（`package` 字段或 key 本身）
  const v = entry.value;
  if (typeof v === 'object' && v !== null) {
    return {
      depKey: entry.key,
      actualName: v.package || entry.key,
      usesWorkspace: v.workspace === true,
      path: v.path || null,
      git: v.git || null,
    };
  }
  return {
    depKey: entry.key,
    actualName: entry.key,
    usesWorkspace: false,
    path: null,
    git: null,
  };
}

function analyzeCrate(localPath, gpuiLearnPath, aacodeCargo, gpuiLearnCargo) {
  const crateName = path.basename(localPath);
  const crateTomlPath = path.join(ROOT, localPath, 'Cargo.toml');

  if (!existsSync(crateTomlPath)) {
    console.error(`❌ ${localPath}/Cargo.toml 不存在`);
    return;
  }

  const crateToml = parseTOML(readFileSync(crateTomlPath, 'utf8'));
  const pkgName = crateToml.package?.name || crateName;
  const members = (aacodeCargo.workspace?.members || []).flatMap((m) => {
    if (m.includes('*')) return []; // skip globs
    return [m];
  });
  const inMembers = members.includes(localPath);

  const gpuiWsDeps = gpuiLearnCargo.workspace?.dependencies || {};
  const gpuiMembers = gpuiLearnCargo.workspace?.members || [];

  console.log(`\n${'═'.repeat(60)}`);
  console.log(`📦 ${localPath}  →  ${pkgName}`);
  console.log(`${'═'.repeat(60)}`);
  console.log(`  在 aacode members 中: ${inMembers ? '✅ 是' : '❌ 否'}`);
  console.log(`  gpui_learn 镜像存在:   ${checkGpuiLearnMirror(localPath, gpuiLearnPath, pkgName, gpuiMembers)}`);

  const deps = walkDeps(crateToml);
  const wsDeps = deps.filter((d) => d.value?.workspace === true || d.usesWorkspace);
  const directDeps = deps.filter((d) => !(typeof d.value === 'object' && d.value !== null && d.value.workspace === true));

  // ── workspace = true 依赖分析 ──
  const missingWs = [];
  const presentWs = [];
  for (const d of wsDeps) {
    const resolved = resolveDepName(d);
    const actualName = resolved.actualName;
    if (gpuiWsDeps[actualName] === undefined) {
      missingWs.push(resolved);
    } else {
      presentWs.push({ ...resolved, gpuiSpec: gpuiWsDeps[actualName] });
    }
  }

  if (wsDeps.length > 0) {
    console.log(`\n📋 workspace = true 依赖 (${wsDeps.length} 个)`);
    console.log(`${'─'.repeat(40)}`);
    if (presentWs.length) {
      console.log(`  ✅ gpui_learn 已有 (${presentWs.length}):`);
      for (const d of presentWs) {
        let spec = '';
        if (d.gpuiSpec.path) spec = `path = "${d.gpuiSpec.path}"`;
        else if (d.gpuiSpec.git) spec = `git = ..., rev = ${d.gpuiSpec.rev?.slice(0, 7) || '?'}`;
        else if (typeof d.gpuiSpec === 'string') spec = `crates.io @ ${d.gpuiSpec}`;
        console.log(`     ${d.actualName}  ←  ${spec}`);
      }
    }
    if (missingWs.length) {
      console.log(`  ❌ gpui_learn 缺失 (${missingWs.length}):`);
      for (const d of missingWs) {
        const source = findSourceInAacode(d.actualName, aacodeCargo);
        console.log(`     ${d.actualName}  (${source})`);
      }
      printMissingSnippet(missingWs, aacodeCargo);
    }
  }

  // ── 非 workspace 依赖（直接声明）──
  if (directDeps.length > 0) {
    console.log(`\n📋 直接声明的依赖 (${directDeps.length} 个)`);
    for (const d of directDeps) {
      const v = d.value;
      if (typeof v === 'string') console.log(`     ${d.section} ${d.key} = "${v}"`);
      else console.log(`     ${d.section} ${d.key} = ${JSON.stringify(v)}`);
    }
  }

  // ── aacode 侧修改建议 ──
  const aliasKey = suggestAlias(pkgName);
  console.log(`\n📝 aacode Cargo.toml 修改:`);
  console.log(`  1. 从 [workspace].members 删: "${localPath}"`);
  console.log(`  2. 把 ${localPath.replace('packages/', 'packages/')} 目录删掉`);
  const gitLine = `  3. workspace.dependencies 加/改:\n     ${aliasKey} = { package = "${pkgName}", git = "https://github.com/Nahida-aa/gpui_learn", rev = "LATEST_REV" }`;
  console.log(gitLine);

  // ── gpui_learn 侧 members 建议 ──
  const gpuiRelPath = suggestGpuiLearnPath(localPath, pkgName);
  const alreadyInGpui = gpuiMembers.some((m) => m.includes(path.basename(gpuiRelPath)));
  console.log(`\n📝 gpui_learn Cargo.toml 修改:`);
  if (!alreadyInGpui) {
    console.log(`  1. [workspace].members 加: "${gpuiRelPath}"`);
  } else {
    console.log(`  1. members 已有 ${path.basename(gpuiRelPath)}，skip`);
  }
  if (pkgName !== aliasKey) {
    console.log(`  2. workspace.dependencies 加 alias:\n     ${aliasKey} = { package = "${pkgName}", path = "${gpuiRelPath}" }`);
  } else {
    console.log(`  2. package 名 = alias，skip`);
  }

  // ── 源码同步建议 ──
  // 查 aacode 里有没有用旧 crate 名的 use 语句
  const srcUsages = findSrcUsages(pkgName, pkgName);
  if (srcUsages.length > 0) {
    console.log(`\n🔤 源码可能需要 crate 名替换:`);
    console.log(`  旧: ${pkgName}  →  新: ${pkgName}`);
    console.log(`  若 gpui_learn 里用的是别的名字，记得全局替换。`);
  }
}

function checkGpuiLearnMirror(localPath, gpuiLearnPath, pkgName, gpuiMembers) {
  // 1. 同路径
  const guesses = [localPath, `packages/zed/${pkgName}`];
  for (const g of guesses) {
    if (existsSync(path.join(gpuiLearnPath, g, 'Cargo.toml'))) {
      return `✅ ${g}`;
    }
  }

  // 2. 按 package name 匹配
  const dirs = ['packages/aa', 'packages/aa_gpui', 'packages/aa_gpui_kit', 'packages/zed', 'packages'];
  for (const d of dirs) {
    const searchDir = path.join(gpuiLearnPath, d);
    if (!existsSync(searchDir)) continue;
    const subdirs = readdirSync(searchDir);
    for (const sub of subdirs) {
      const ct = path.join(searchDir, sub, 'Cargo.toml');
      if (!existsSync(ct)) continue;
      try {
        const ctText = readFileSync(ct, 'utf8');
        const m = ctText.match(/name\s*=\s*"([^"]+)"/);
        if (m && m[1] === pkgName) return `✅ 存在于 ${d}/${sub}/ (name = "${pkgName}")`;
      } catch { /* skip */ }
    }
  }

  // 3. 按目录名匹配（忽略 package 名）—— 目录名可能是别名（如 zed_paths 对应 paths）
  for (const d of dirs) {
    const searchDir = path.join(gpuiLearnPath, d);
    if (!existsSync(searchDir)) continue;
    const subdirs = readdirSync(searchDir);
    for (const sub of subdirs) {
      const lname = sub.toLowerCase();
      const lpkg = pkgName.toLowerCase();
      // 匹配: zed_paths → paths, aa_gpui_kit_fs → fs, 或直接同名
      if (lname === lpkg || lname.endsWith(`_${lpkg}`) || lname.startsWith(`${lpkg}_`)) {
        const ct = path.join(searchDir, sub, 'Cargo.toml');
        if (existsSync(ct)) return `✅ 可能是 ${d}/${sub}/ (目录名匹配: ${sub} ~ ${pkgName})`;
      }
    }
  }

  return '❌ 未找到，需手动复制源码';
}

function suggestAlias(pkgName) {
  // aacode workspace.dependencies key 通常就是 package 名本身
  return pkgName;
}

function suggestGpuiLearnPath(localPath, pkgName) {
  // 猜测 gpui_learn 里的位置（最简化）
  const base = path.basename(localPath);
  return `packages/zed/${base}`; // 常见位置
}

function findSourceInAacode(actualName, aacodeCargo) {
  const wsDeps = aacodeCargo.workspace?.dependencies || {};
  if (wsDeps[actualName]) {
    const spec = wsDeps[actualName];
    if (spec.git) return `aacode git dep @ ${spec.git.split('/').pop()} rev ${spec.rev?.slice(0, 7) || '?'}`;
    if (spec.path) return `aacode path dep @ ${spec.path}`;
    if (typeof spec === 'string') return `aacode crates.io @ ${spec}`;
    return `aacode ws.dep (unknown)`;
  }
  return `❓ 不在 aacode workspace.dependencies，可能是 zed patch 或别的来源`;
}

function printMissingSnippet(missingWs, aacodeCargo) {
  console.log(`\n  ── 可直接复制到 gpui_learn workspace.dependencies: ──`);
  console.log(`  # 补 ${missingWs.length} 个缺失依赖:`);
  const wsDeps = aacodeCargo.workspace?.dependencies || {};
  for (const d of missingWs) {
    const spec = wsDeps[d.actualName];
    let line;
    if (typeof spec === 'string') {
      line = `${d.actualName} = "${spec}"`;
    } else if (spec.path) {
      line = `${d.actualName} = { path = "${spec.path}" }`;
    } else if (spec.git) {
      const ver = spec.version ? `, version = "${spec.version}"` : '';
      line = `${d.actualName} = { git = "${spec.git}", rev = "${spec.rev}"${ver} }`;
    } else {
      const parts = [];
      if (spec.version) parts.push(`version = "${spec.version}"`);
      if (spec.features) parts.push(`features = [${spec.features.map((f) => `"${f}"`).join(', ')}]`);
      line = `${d.actualName} = { ${parts.join(', ')} }`;
    }
    console.log(`  ${line}`);
  }
}

function findSrcUsages(oldName, newName) {
  try {
    const out = execSync(`rg '\\b${oldName}::' packages/ --glob='*.rs' -l --no-heading 2>/dev/null`, {
      encoding: 'utf8',
      cwd: ROOT,
    });
    return out.split('\n').filter(Boolean);
  } catch { return []; }
}

// ── 入口 ────────────────────────────────────────────────────────────────────

const args = process.argv.slice(2);
let gpuiLearnPath = '';
const cratePaths = [];

for (let i = 0; i < args.length; i++) {
  if (args[i] === '--gpui-learn') {
    gpuiLearnPath = path.resolve(args[++i]);
  } else {
    cratePaths.push(args[i]);
  }
}

if (!gpuiLearnPath) {
  // 猜测：优先用用户常用路径
  const guesses = [
    path.join(path.dirname(ROOT), '..', 'ide_ls', 'gpui_learn'),
    path.join(path.dirname(ROOT), 'gpui_learn'),
  ];
  for (const g of guesses) {
    if (existsSync(path.join(g, 'Cargo.toml'))) {
      gpuiLearnPath = g;
      break;
    }
  }
}

if (!existsSync(path.join(gpuiLearnPath, 'Cargo.toml'))) {
  console.error('❌ 找不到 gpui_learn，请用 --gpui-learn /path/to/gpui_learn');
  process.exit(1);
}

if (!cratePaths.length) {
  console.error('用法: node tools/analyze-crate.mjs packages/paths [--gpui-learn /path]');
  process.exit(1);
}

const aacodeCargo = parseTOML(readFileSync(path.join(ROOT, 'Cargo.toml'), 'utf8'));
const gpuiLearnCargo = parseTOML(readFileSync(path.join(gpuiLearnPath, 'Cargo.toml'), 'utf8'));

console.log(`\n🔍 aacode ↔ gpui_learn 迁移分析`);
console.log(`   aacode:     ${ROOT}`);
console.log(`   gpui_learn: ${gpuiLearnPath}`);

for (const cp of cratePaths) {
  analyzeCrate(cp, gpuiLearnPath, aacodeCargo, gpuiLearnCargo);
}

console.log(`\n${'═'.repeat(60)}`);
console.log('✅ 完成');
console.log(`${'═'.repeat(60)}`);
