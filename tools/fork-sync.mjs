#!/usr/bin/env bun
import { $ } from 'bun';
import { existsSync, readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { join, basename } from 'node:path';

const ROOT = process.cwd();
const OUTDIR = '.agents/fork-sync';
const ZD = process.env.ZED_REPO || '/home/aa/repos/ide_ls/learn_ls/zed';

function parseArgs(argv) {
  const args = { pos: [] };
  for (let i = 2; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--list') args.list = true;
    else if (a === '--full') args.full = true;
    else if (a === '--old') args.old = argv[++i];
    else if (a === '--new') args.new = argv[++i];
    else if (a === '--batch') args.batch = argv[++i];
    else if (a === '--note') args.note = argv[++i];
    else if (a === '--crate') args.crate = argv[++i];
    else if (!a.startsWith('-')) args.pos.push(a);
  }
  return args;
}

async function main() {
  const args = parseArgs(process.argv);
  if (args.list) {
    try {
      const dir = OUTDIR;
      if (!existsSync(dir)) return;
      const files = readdirSync(dir).filter(f => f.endsWith('.md') && f !== 'README.md');
      console.log('同步计划（' + ROOT + '/' + OUTDIR + '）：');
      for (const bn of files) {
        const f = join(dir, bn);
        const txt = readFileSync(f, 'utf8');
        const m = txt.match(/^# 同步 ([^\n]+)/m);
        const m2 = txt.match(/🟡[^|]*|🟢[^|]*|🔴[^|]*|⚪[^|]*/);
        console.log('  ' + bn.padEnd(28) + ' ' + ((m2 && m2[0]) || '?').trim().padEnd(10) + ' ' + (m ? m[1] : ''));
      }
    } catch (e) {}
    return;
  }
  if (args.full) {
    console.log('[fork-sync] --full: crate级全量 + 文件级全量（按 crate 逐个处理）');
    console.log('说明：遍历所有 fork crate 的所有文件，对比 zed old vs aacode 当前文件，生成 L0/L1/L2 盘点');
    console.log('可配合 --crate 限定单个 crate。扫描顺序应遵循计划文件维护的优先级（基础→上层）。');
    const c = args.crate || (args.pos.length > 0 ? args.pos[0] : '');
    if (c) console.log('--crate: ' + c);
    if (c === 'project') console.log('本次仅对 project 做全量（crate 内文件级全量）');
    return;
  }
  console.log('fork-sync.mjs: use --list or --full');
}

main();
