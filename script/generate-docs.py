#!/usr/bin/env python3
"""Generate markdown API reference from rustdoc JSON.

Mirrors the layout of the upstream TanStack Workflow docs/reference/
(typedoc-plugin-markdown output): one file per item, frontmatter
`id`/`title`, `index.md` with per-kind sections, and per-item
Type Parameters / Parameters / Returns sections with cross-item links.

Usage:
    python3 script/generate-docs.py --crate aa-core [--no-build] [--json PATH] [--out DIR]

Requires nightly for rustdoc JSON (`rustup run nightly cargo rustdoc ...`),
unless --no-build/--json reuse an existing target/doc/*.json.
Output: docs/reference/<crate-slug>/ (committed, like upstream).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

DEFAULT_CRATE = "aa_workflow_core"
EMPTY_G = {"params": [], "where_predicates": []}

# (repo, filename) → 源码行缓存（仅常量/static 回读值用）
_source_line_cache: dict[tuple[str, str], list[str]] = {}
# main() 里设置：回读源码定义行用的仓库根
_SOURCE_REPO: Path | None = None

# inner kind -> (dirname, page title prefix)
KIND_DIRS = {
    "struct": ("structs", "Struct"),
    "enum": ("enums", "Enum"),
    "trait": ("traits", "Trait"),
    "function": ("functions", "Function"),
    "type_alias": ("type-aliases", "Type Alias"),
    "constant": ("constants", "Constant"),
    "static": ("constants", "Static"),
}

# item id -> (dirname, slug)，渲染期填充
LINK_TARGETS: dict[int, tuple[str, str]] = {}


def assign_slugs(found: dict[str, list[dict]]) -> dict[int, str]:
    """给每个 item 定文件名：不重名用 item 名，重名用 <模块路径>::<item 名>.md 消歧。
    取 item 所在模块的尾部路径（1 → 全路径，逐级追加），保证组内唯一。"""
    by_name: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for kind, items in found.items():
        for it in items:
            by_name[(kind, it["name"] or "")].append(it)
    slugs: dict[int, str] = {}
    for (kind, name), group in by_name.items():
        if len(group) == 1:
            slugs[group[0]["id"]] = name
            continue
        cand = [name] * len(group)
        # 逐级取模块路径尾部 1..n 段（不含 crate 根段），直到组内唯一
        for nseg in range(1, 64):  # 64 段足够深；超过则自然取全路径
            cand = []
            for it in group:
                mod_segs = it["_path"][1:]                      # 去掉 crate 根段
                tail = mod_segs[-nseg:] if nseg < len(mod_segs) else mod_segs
                cand.append("::".join(tail + [name]) if tail else name)
            if len(set(cand)) == len(group):
                break
        for it, s in zip(group, cand):
            slugs[it["id"]] = s
    # 全路径仍撞名（理论上不会，比如同 item 多路注册）时追加序号兑底
    for kind, items in found.items():
        used: dict[str, int] = defaultdict(int)
        for it in items:
            s = slugs[it["id"]]
            used[s] += 1
            if used[s] > 1:
                s = f"{s}_{used[s]}"
            slugs[it["id"]] = s
    return slugs


def json_path(repo: Path, crate: str, json_arg: str | None) -> Path:
    if json_arg:
        p = Path(json_arg)
        return p if p.is_absolute() else repo / p
    # rustdoc JSON 按 lib target 命名：crate 名的 - 换成 _
    candidates = [repo / "target" / "doc" / f"{c.replace('-', '_')}.json" for c in (crate, crate.replace("-", "_"))]
    for c in candidates:
        if c.exists():
            return c
    return candidates[0]


def build_json(repo: Path, crate: str, json_p: Path, no_build: bool) -> None:
    if no_build and json_p.exists():
        return
    cargo = shutil.which("cargo") or "cargo"
    subprocess.run(
        [
            "rustup", "run", "nightly", cargo, "rustdoc",
            "-p", crate, "--", "-Z", "unstable-options",
            "--output-format", "json",
        ],
        cwd=repo, check=True,
    )


def last_seg(path: str) -> str:
    return path.rsplit("::", 1)[-1]


def sub_level(level: str) -> str:
    return "#" + level


def resolve_link(tgt, here: str) -> str | None:
    """item id → 相对本页的交叉链接；解析不了返回 None。
    所有 item 页都在 out/<kind>/ 下，跨 kind 只需 1 层 ../ 回到 out/。"""
    hit = LINK_TARGETS.get(tgt) if isinstance(tgt, int) else None
    if not hit:
        return None
    d, name = hit
    if d == here:
        return f"{name}.md"
    return f"../{d}/{name}.md"


def render_type(t) -> str:
    """Best-effort signature rendering from a rustdoc-json type tree."""
    if t is None:
        return "()"
    if "generic" in t:
        return t["generic"]
    if "primitive" in t:
        return t["primitive"]
    if "resolved_path" in t:
        rp = t["resolved_path"]
        s = last_seg(rp["path"])
        args = rp.get("args")
        if args and "angle_bracketed" in args:
            parts = []
            for a in args["angle_bracketed"].get("args") or []:
                if "type" in a:
                    parts.append(render_type(a["type"]))
                elif "lifetime" in a:
                    lt = a["lifetime"]
                    parts.append(lt if isinstance(lt, str)
                                 else ((lt.get("args") or ["'_"])[0]
                                       if isinstance(lt.get("args"), list) and lt.get("args") else "'_"))
            if parts:
                s += "<" + ", ".join(parts) + ">"
        return s
    if "borrowed_ref" in t:
        br = t["borrowed_ref"]
        mut = "mut " if br.get("mutable") else ""
        lt = br.get("lifetime") or ""
        return f"&{lt + ' ' if lt else ''}{mut}{render_type(br['type'])}"
    if "tuple" in t:
        types = t["tuple"] if isinstance(t["tuple"], list) else t["tuple"].get("types") or []
        return "()" if not types else "(" + ", ".join(render_type(x) for x in types) + ")"
    if "slice" in t:
        s = t["slice"]
        # format 60：{"slice": <type>}；旧版 {"slice": {"type": ...}}
        return f"[{render_type(s.get('type') if isinstance(s, dict) and 'type' in s else s)}]"
    if "array" in t:
        a = t["array"]
        elem = a.get("type") if isinstance(a, dict) and "type" in a else a
        return f"[{render_type(elem)}; {a.get('len', 'N') if isinstance(a, dict) else 'N'}]"
    if "dyn_trait" in t:
        trs = " + ".join(
            last_seg(x["trait"]["path"]) + render_args_paren(x["trait"].get("args"))
            for x in t["dyn_trait"]["traits"]
        )
        return f"dyn {trs}"
    if "impl_trait" in t:
        # format 60 里 impl_trait 直接是 bounds 数组（旧版是 {"trait_bounds": [...]}）
        bounds = t["impl_trait"] if isinstance(t["impl_trait"], list) else t["impl_trait"]["trait_bounds"]
        return "impl " + " + ".join(
            render_type({"resolved_path": b["trait"]}) if isinstance(b, dict) and "trait" in b else "?"
            for b in bounds
        )
    if "function_pointer" in t:
        return render_fn_sig("fn", t["function_pointer"]["sig"], None)
    if "qualified_path" in t:
        return t["qualified_path"].get("name", "?")
    if "unnamed" in t:
        return "_"
    return "?"  # 未知形态的兜底


def render_args_paren(args) -> str:
    """Parenthesized args（Fn(A) -> B 形态）。"""
    if not args or "parenthesized" not in args:
        return ""
    p = args["parenthesized"]
    ins = ", ".join(render_type(x) for x in p.get("inputs") or [])
    out = p.get("output")
    return f"({ins})" + (f" -> {render_type(out)}" if out else "")


def render_type_linked(t, here: str) -> str:
    """Section（代码块外）用：可解析到本 reference 页的类型渲染成链接。"""
    if isinstance(t, dict) and "resolved_path" in t:
        tgt = resolve_link(t["resolved_path"].get("id"), here)
        if tgt:
            return f"[`{render_type(t)}`]({tgt})"
    return f"`{render_type(t)}`"


def render_bounds_linked(bounds: list, here: str) -> str:
    parts = []
    for b in bounds or []:
        if not isinstance(b, dict):
            parts.append(str(b))
            continue
        tb = b.get("trait_bound") or (b if "trait" in b else None)
        if tb and "trait" in tb:
            tr = tb["trait"]
            txt = last_seg(tr["path"]) + render_args_paren(tr.get("args"))
            tgt = resolve_link(tr.get("id"), here)
            parts.append(f"[`{txt}`]({tgt})" if tgt else f"`{txt}`")
        elif "outlives" in b:
            parts.append(b["outlives"])
    return " + ".join(parts)


def render_generics(g: dict) -> tuple[str, str]:
    """返回 (<>内参数, where 子句) 的 best-effort 渲染（代码块内纯文本）。"""
    params = []
    for p in g.get("params") or []:
        k = p.get("kind") or {}
        name = p.get("name") or ""
        if "lifetime" in k:
            params.append(name)
            continue
        bounds = render_bounds_linked(k.get("type", {}).get("bounds") if "type" in k else [], "")
        bounds = bounds.replace("`", "")
        params.append(f"{name}: {bounds}" if bounds else name)
    wheres = []
    for wp in g.get("where_predicates") or []:
        bp = wp.get("bound_predicate") or wp.get("bound") or {}
        ty = render_type(bp.get("type"))
        bs = render_bounds_linked(bp.get("bounds") or [], "").replace("`", "")
        if bs:
            wheres.append(f"{ty}: {bs}")
    gen = f"<{', '.join(params)}>" if params else ""
    where = f"\nwhere\n    {',\n    '.join(wheres)}\n" if wheres else ""
    return gen, where


def render_fn_sig(name: str, sig: dict, header: dict | None) -> str:
    h = header or {}
    kw = ""
    if h.get("is_const"):
        kw += "const "
    if h.get("is_async"):
        kw += "async "
    if h.get("is_unsafe"):
        kw += "unsafe "
    inputs = sig.get("inputs") or []
    rendered = []
    for i, (n, t) in enumerate(inputs):
        if i == 0 and n == "self":
            # 折回惯用 self 形态：&Self → &self，&mut Self → &mut self
            if isinstance(t, dict) and "borrowed_ref" in t:
                br = t["borrowed_ref"]
                if isinstance(br.get("type"), dict) and br["type"].get("generic") == "Self":
                    rendered.append("&mut self" if br.get("mutable") else "&self")
                    continue
            if isinstance(t, dict) and t.get("generic") == "Self":
                rendered.append("self")
                continue
        rendered.append(f"{n}: {render_type(t)}")
    args = ", ".join(rendered)
    out = sig.get("output")
    ret = f" -> {render_type(out)}" if out else ""
    return f"{kw}fn {name}({args}){ret}"


def non_self_inputs(sig: dict) -> list:
    return [(n, t) for i, (n, t) in enumerate(sig.get("inputs") or [])
            if not (i == 0 and n == "self")]


class Docs:
    def __init__(self, data: dict):
        self.idx: dict[int, dict] = {int(k): v for k, v in data["index"].items()}
        self.root_id = int(data["root"])
        self.reexport_ids: set[int] = set()

    def collect_public(self) -> dict[str, list[dict]]:
        """从 crate root 递归收集 public items（含 re-export，按 id 去重）。

        - 显式 `pub use` 跟随目标 item；glob `pub use M::*` 展开为对 M 的递归收集。
        - 每个 item 附带 `_path`（所属模块名列表，根为 crate 名）供重名 item 的文件名消歧。
        - 经 `pub use` 进入的 item 记入 `self.reexport_ids`，供生成 re-exports.md。
        """
        found: dict[str, dict[int, dict]] = defaultdict(dict)
        seen: set[int] = set()
        reexports: set[int] = set()
        best_path: dict[int, list] = {}  # item id → 最短模块路径（多条路可达时优先短路径）
        crate = self.idx[self.root_id].get("name") or "crate"

        def register(it: dict, path: list, via_use: bool) -> None:
            if via_use:
                reexports.add(it["id"])
            cur = best_path.get(it["id"])
            if cur is None or len(path) < len(cur):
                best_path[it["id"]] = path
            for kind in KIND_DIRS:
                if kind in it["inner"]:
                    found[kind][it["id"]] = it
                    break

        def walk(module_id: int, path: list, via_use: bool = False) -> None:
            mod = self.idx[module_id]["inner"]["module"]
            for iid in mod.get("items") or []:
                if iid in seen:
                    continue
                seen.add(iid)
                it = self.idx[iid]
                if it["visibility"] != "public":
                    continue
                inner = it["inner"]
                if "use" in inner:
                    # 再导出：以目标 id 为 key，若同一 item 也能经 pub module 走到，
                    # 两路落到同一 key，天然去重。
                    u = inner["use"]
                    if u.get("is_glob"):
                        # 展开 glob：目标若是本 crate 内的模块，递归收集其 public 项
                        tid = u.get("id")
                        if tid is not None and tid in self.idx and "module" in self.idx[tid]["inner"]:
                            walk(tid, path + [self.idx[tid].get("name") or "?"], via_use=True)
                        continue
                    if u.get("id") is None or u["id"] not in self.idx:
                        continue
                    register(self.idx[u["id"]], path, via_use=True)
                    continue
                if "module" in inner:
                    walk(iid, path + [it["name"]], via_use)
                else:
                    register(it, path, via_use)

        walk(self.root_id, [crate])
        # 多条路可达的 item 用最短路径，保证直接重导出的 item 保留无模块前缀的干净名
        for kind_items in found.values():
            for it in kind_items.values():
                it["_path"] = best_path[it["id"]]
        self.reexport_ids = reexports
        return {k: sorted(v.values(), key=lambda x: x["name"] or "") for k, v in found.items()}


def build_link_targets(docs: Docs, found: dict[str, list[dict]], slugs: dict[int, str]) -> None:
    """item id → (dirname, slug)。方法/变体等子 item 指到父页面。"""
    for kind, items in found.items():
        d = KIND_DIRS[kind][0]
        for it in items:
            s = slugs[it["id"]]
            LINK_TARGETS[it["id"]] = (d, s)
            inner = it["inner"][kind]
            if kind in ("struct", "enum"):
                for iid in inner.get("impls") or []:
                    imp = docs.idx[iid]["inner"]["impl"]
                    for mid in imp.get("items") or []:
                        m = docs.idx[mid]
                        if "function" in m["inner"]:
                            LINK_TARGETS[m["id"]] = (d, s)
                if kind == "enum":
                    for vid in inner.get("variants") or []:
                        LINK_TARGETS[vid] = (d, s)
            elif kind == "trait":
                for iid in inner.get("items") or []:
                    LINK_TARGETS[iid] = (d, s)


def expand_docs(it: dict, here: str) -> str:
    """把 doc 注释里的 rustdoc intra-doc link 展开成相对 markdown 链接。

    links 字典的 key 有两种存法（value 都是目标 item id）：
    - shortcut 形式 `[`Foo`]`：key 就是 "`Foo`"（含反引号）
    - 带路径形式 `[`text`](Foo::bar)`：key 是路径部分 "Foo::bar"
    解析不了的目标退化为纯 code 文本（去掉方括号/链接）。
    """
    text = it.get("docs")
    if not text:
        return ""
    for key, tgt in (it.get("links") or {}).items():
        target = resolve_link(tgt, here)
        esc = re.escape(key)
        if target:
            # 带路径形式：保留原文本，换 url
            text = re.sub(r"\[([^\]]+)\]\(" + esc + r"\)",
                          lambda m: f"[{m.group(1)}]({target})", text)
            # shortcut 形式
            text = text.replace("[" + key + "]", f"[{key}]({target})")
        else:
            text = re.sub(r"\[([^\]]+)\]\(" + esc + r"\)", lambda m: m.group(1), text)
            text = text.replace("[" + key + "]", key)
    return text.rstrip() + "\n\n"


def _source_value(it: dict) -> str:
    """非字面量常量的 JSON 里 expr 是占位符 "_"，回读源码定义取 “=” 右侧的值。
    定义可能跨多行（“=” 在行尾、值在下一行），所以拼接整个 span 的行再截取。"""
    sp = it.get("span")
    if not sp or _SOURCE_REPO is None:
        return ""
    fname = sp["filename"]
    key = (str(_SOURCE_REPO), fname)
    lines = _source_line_cache.get(key)
    if lines is None:
        path = _SOURCE_REPO / fname
        if not path.exists():
            return ""
        lines = path.read_text(encoding="utf-8").splitlines()
        _source_line_cache[key] = lines
    start, end = sp["begin"][0], sp["end"][0]
    if start < 1 or start > len(lines):
        return ""
    end = min(end, len(lines))
    joined = " ".join(ln.strip() for ln in lines[start - 1:end])
    # 取首个 “=” 作为赋值号（const/static 定义前缀 pub const NAME: TYPE 不含 “=”）
    eq = joined.find("=")
    if eq < 0:
        return ""
    val = joined[eq + 1 :].strip()
    # 去掉末尾分号
    if val.endswith(";"):
        val = val[:-1].rstrip()
    return val


def defined_in(it: dict, depth: int) -> str:
    sp = it.get("span")
    if not sp:
        return ""
    fname, line = sp["filename"], sp["begin"][0]
    prefix = "" if depth == 0 else "../" * depth
    return f"Defined in: [`{fname}:{line}`]({prefix}{fname}#L{line})\n"


def write_page(path: Path, slug: str, title: str, body: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(f"---\nid: {slug}\ntitle: {slug}\n---\n\n# {title}\n\n{body}", encoding="utf-8")


def methods_of(docs: Docs, it: dict) -> tuple[list[dict], list[str], list[str]]:
    """返回 (固有方法 items, 非合成 trait impl 名列表, 合成 trait 名列表)。"""
    inner = it["inner"].get("struct") or it["inner"].get("enum") or {}
    methods, trait_impls, synthetic = [], [], []
    for iid in inner.get("impls") or []:
        imp = docs.idx[iid]["inner"]["impl"]
        if imp.get("is_negative"):
            continue
        if imp.get("trait") is None:
            for mid in imp.get("items") or []:
                m = docs.idx[mid]
                if "function" in m["inner"] and m["visibility"] == "public":
                    methods.append(m)
        else:
            tname = last_seg(imp["trait"]["path"])
            if imp.get("is_synthetic"):
                synthetic.append(tname)
            else:
                trait_impls.append(tname)
    return methods, trait_impls, synthetic


def fn_section(docs: Docs, it: dict, here: str, depth: int, level: str = "###") -> str:
    """方法/函数条目：标题 + 签名 + Defined in + docs + Parameters + Returns。"""
    f = it["inner"]["function"]
    name = it["name"] or "_"
    gen, where = render_generics(f.get("generics") or EMPTY_G)
    sig = render_fn_sig(name, f["sig"], f.get("header"))
    if gen:
        sig = sig.replace(f"fn {name}(", f"fn {name}{gen}(", 1)
    if where:
        sig += where.rstrip("\n")
    out = f"{level} {name}()\n\n```rust\npub {sig}\n```\n\n"
    di = defined_in(it, depth)
    if di:
        out += di + "\n"
    out += expand_docs(it, here)
    sub, entry = sub_level(level), sub_level(sub_level(level))
    params = non_self_inputs(f["sig"])
    if params:
        out += f"{sub} Parameters\n\n"
        for n, t in params:
            out += f"{entry} {n}\n\n{render_type_linked(t, here)}\n\n"
    o = f["sig"].get("output")
    if o:
        out += f"{sub} Returns\n\n{render_type_linked(o, here)}\n\n"
    return out


def type_params_section(g: dict, here: str, level: str = "##") -> str:
    ps = []
    for p in g.get("params") or []:
        k = p.get("kind") or {}
        name = p.get("name") or ""
        if "type" in k:
            ps.append((name, render_bounds_linked(k["type"].get("bounds") or [], here)))
        elif "const" in k:
            ps.append((name, ""))
    if not ps:
        return ""
    out = f"{level} Type Parameters\n\n"
    for n, b in ps:
        out += f"{sub_level(level)} {n}\n\n`{n}`" + (f" *extends* {b}" if b else "") + "\n\n"
    return out


def enum_definition_block(it: dict, docs: Docs) -> str:
    """原始 enum 定义：每个 variant 一行，内联 /// 注释。"""
    inner = it["inner"]["enum"]
    sk = norm_kind(inner.get("kind"))
    variants = inner.get("variants") or sk.get("variants") or []
    g = inner.get("generics") or {}
    gen = ""
    if g.get("params"):
        ps = [p.get("name") or "" for p in g["params"]]
        gen = "<" + ", ".join(ps) + ">"
    lines = [f"pub enum {it['name'] or '?'}{gen}"]
    if variants:
        lines[-1] += " {"
        for vid in variants:
            v = docs.idx[vid]
            vk = v["inner"].get("variant", {}).get("kind") or {}
            payload = ""
            if "tuple" in vk:
                fs = vk["tuple"] if isinstance(vk["tuple"], list) else (vk["tuple"].get("fields") or [])
                tys = []
                for x in fs:
                    sf = docs.idx[x]["inner"].get("struct_field") if isinstance(x, int) else x
                    tys.append(render_type(sf.get("type") if isinstance(sf, dict) and "type" in sf else sf))
                payload = "(" + ", ".join(tys) + ")"
            elif "struct" in vk:
                payload = "{ .. }"
            doc = _inline_doc(v.get("docs") or "")
            prefix = f"    // {doc}\n" if doc else ""
            lines.append(prefix + f"    {v['name']}{payload},")
        lines.append("}")
    return "```rust\n" + "\n".join(lines) + "\n```\n\n"


def norm_kind(sk):
    """kind 字段两种形态："plain" / {"plain": {...}}（旧版）。统一成 dict。"""
    if isinstance(sk, str):
        sk = {sk: {}}
    return sk or {}


def _inline_doc(doc: str) -> str:
    """把多行 doc 注释压成一行，供 Definition 代码块内联展示。"""
    return " ".join(doc.split())


def struct_definition_block(it: dict, docs: Docs) -> str:
    """原始 struct 定义（含逐字段行与内联 /// 注释），代码块内纯文本。"""
    inner = it["inner"]["struct"]
    sk = norm_kind(inner.get("kind"))
    fields = sk.get("plain", {}).get("fields") or sk.get("union", {}).get("fields") or []
    kw = "union" if "union" in sk else "struct"
    gen = ""
    g = inner.get("generics") or {}
    if g.get("params"):
        # 只需 <> 参数（where 子句另列）
        ps = []
        for p in g["params"]:
            k = p.get("kind") or {}
            ps.append(p.get("name") or "")
        gen = "<" + ", ".join(ps) + ">"
    lines = [f"pub {kw} {it['name'] or '?'}{gen}"]
    if fields:
        lines[-1] += " {"
        for fid in fields:
            fl = docs.idx[fid]
            sf = fl["inner"].get("struct_field")
            ft = sf.get("type") if isinstance(sf, dict) and "type" in sf else sf
            vis = "pub " if fl["visibility"] == "public" else ""
            doc = _inline_doc(fl.get("docs") or "")
            prefix = f"    // {doc}\n" if doc else ""
            lines.append(prefix + f"    {vis}{fl['name']}: {render_type(ft)},")
        lines.append("}")
    return "```rust\n" + "\n".join(lines) + "\n```\n\n"


def render_item_page(docs: Docs, kind: str, it: dict, here: str, depth: int) -> str:
    inner = it["inner"][kind]
    body = defined_in(it, depth) + "\n" + expand_docs(it, here)

    if kind == "struct":
        sk = norm_kind(inner.get("kind"))
        body += "## Definition\n\n" + struct_definition_block(it, docs)
        fields = sk.get("plain", {}).get("fields") or []
        doc_fields = [docs.idx[fid] for fid in fields if docs.idx[fid].get("docs")]
        if doc_fields:
            entries = []
            for fl in doc_fields:
                e = f"### {fl['name']}\n\n"
                di = defined_in(fl, depth)
                if di:
                    e += di + "\n"
                e += expand_docs(fl, here)
                entries.append(e)
            body += "## Fields\n\n" + "\n***\n\n".join(entries)
        if sk.get("plain", {}).get("has_stripped_fields") or sk.get("union", {}).get("has_stripped_fields"):
            body += "_（存在非公开字段）_\n\n"
        methods, trait_impls, synthetic = methods_of(docs, it)

    elif kind == "enum":
        sk = norm_kind(inner.get("kind"))
        variants = sk.get("variants") or inner.get("variants") or []
        body += "## Definition\n\n" + enum_definition_block(it, docs)
        doc_variants = [docs.idx[vid] for vid in variants if docs.idx[vid].get("docs")]
        if doc_variants:
            entries = []
            for v in doc_variants:
                e = f"### {v['name']}\n\n"
                di = defined_in(v, depth)
                if di:
                    e += di + "\n"
                e += expand_docs(v, here)
                entries.append(e)
            body += "## Variants\n\n" + "\n***\n\n".join(entries)
        methods, trait_impls, synthetic = methods_of(docs, it)

    elif kind == "trait":
        required, provided = [], []
        for iid in inner.get("items") or []:
            m = docs.idx[iid]
            if "function" not in m["inner"] or m["visibility"] != "public":
                continue
            (provided if m["inner"]["function"].get("has_body") else required).append(m)
        if required:
            body += "## Required Methods\n\n" + "\n***\n\n".join(fn_section(docs, m, here, depth) for m in required)
        if provided:
            body += "## Provided Methods\n\n" + "\n***\n\n".join(fn_section(docs, m, here, depth) for m in provided)
        methods, trait_impls, synthetic = [], [], []

    else:  # function / type_alias / constant / static
        if kind == "function":
            # 上游顺序：标题 → 签名 → Defined in → docs → 各分节
            gen, where = render_generics(inner.get("generics") or EMPTY_G)
            sig = render_fn_sig(it["name"] or "_", inner["sig"], inner.get("header"))
            if gen:
                sig = sig.replace(f"fn {it['name']}(", f"fn {it['name']}{gen}(", 1)
            if where:
                sig += where.rstrip("\n")
            body = "```rust\npub " + sig + "\n```\n\n"
            body += defined_in(it, depth) + "\n"
            body += expand_docs(it, here)
            body += type_params_section(inner.get("generics") or EMPTY_G, here)
            params = non_self_inputs(inner["sig"])
            if params:
                body += "## Parameters\n\n"
                for n, t in params:
                    body += f"### {n}\n\n{render_type_linked(t, here)}\n\n"
            o = inner["sig"].get("output")
            if o:
                body += f"## Returns\n\n{render_type_linked(o, here)}\n\n"
            return body
        if kind == "type_alias":
            body += "## Definition\n\n```rust\npub type " + (it["name"] or "?") + " = " + render_type(inner.get("type")) + "\n```\n\n"
        if kind in ("constant", "static"):
            kw = "const" if kind == "constant" else "static"
            body += "## Definition\n\n```rust\npub " + kw + " " + (it["name"] or "?") + ": " + render_type(inner.get("type"))
            # 值在 JSON 里（源码原文）：constant → inner.const.expr；static → inner.expr。
            # 旧版 rustdoc 把这两层包在 inner["constant"]/inner["static"] 里，两种都兼容。
            expr = None
            for base in (inner, inner.get("constant") or {}, inner.get("static") or {}):
                c = base.get("const")
                if isinstance(c, dict) and c.get("expr"):
                    expr = c["expr"]
                    break
                if isinstance(c, str) and c:
                    expr = c
                    break
                e = base.get("expr")
                if e:
                    expr = e
                    break
            # rustdoc 对非字面量常量存的是占位符 "_"，此时回读源码定义行取 “=” 右侧
            if not expr or expr == "_":
                expr = _source_value(it)
            if expr:
                body += " = " + expr
            body += "\n```\n\n"
        methods, trait_impls, synthetic = [], [], []

    if kind in ("struct", "enum"):
        if methods:
            body += "## Implementations\n\n" + "\n***\n\n".join(fn_section(docs, m, here, depth) for m in methods)
        if trait_impls:
            body += "## Trait Implementations\n\n"
            body += "".join(f"- `impl {t} for {it['name']}`\n" for t in trait_impls)
            body += "\n"
        if synthetic:
            body += "## Auto Trait Implementations\n\n"
            body += " ".join(f"`{t}`" for t in sorted(set(synthetic))) + "\n\n"
    return body


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--crate", default=DEFAULT_CRATE, help=f"cargo package 名（默认 {DEFAULT_CRATE}）")
    ap.add_argument("--repo", default=str(Path(__file__).resolve().parent.parent),
                   help="仓库根目录（默认脚本上级目录）")
    ap.add_argument("--json", default=None, help="直接复用已有的 rustdoc JSON 路径（相对路径按 --repo 解析）")
    ap.add_argument("--out", default=None, help="输出目录（默认 <repo>/docs/reference/<crate-slug>）")
    ap.add_argument("--no-build", action="store_true", help="跳过 cargo rustdoc，复用已有 target/doc/*.json")
    args = ap.parse_args()

    repo = Path(args.repo).resolve()
    global _SOURCE_REPO
    _SOURCE_REPO = repo
    crate_slug = args.crate.replace("-", "_")
    json_p = json_path(repo, args.crate, args.json)
    if args.out:
        out = Path(args.out)
        out = out if out.is_absolute() else repo / out
    else:
        out = repo / "docs" / "reference" / crate_slug

    build_json(repo, args.crate, json_p, args.no_build)

    if not json_p.exists():
        sys.exit(f"错误：找不到 rustdoc JSON {json_p}（且未生成成功）")

    data = json.loads(json_p.read_text())
    docs = Docs(data)
    found = docs.collect_public()
    slugs = assign_slugs(found)
    build_link_targets(docs, found, slugs)

    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)

    index_sections = []
    for kind in ("struct", "enum", "trait", "function", "type_alias", "constant", "static"):
        items = found.get(kind) or []
        if not items:
            continue
        dirname, prefix = KIND_DIRS[kind]
        # 页内 Defined in 链接：从页面所在目录（out/<kind>/）到仓库根需要几层 ../
        page_dir = out / dirname
        rel_to_root = os.path.relpath(repo, page_dir)
        depth = 0 if rel_to_root in (".", "") else rel_to_root.count(os.sep) + 1
        plural = {"struct": "Structs", "enum": "Enums", "trait": "Traits", "function": "Functions",
                  "type_alias": "Type Aliases", "constant": "Constants", "static": "Statics"}[kind]
        section = f"## {plural}\n\n"
        for it in items:
            s = slugs[it["id"]]
            name = it["name"]
            # 函数页标题带 ()：# Function: createWorkflow()；带模块前缀的重名函数不加
            bare = s == name
            label = name if bare else s
            title = f"{prefix}: {label}()" if kind == "function" and bare else f"{prefix}: {label}"
            page = render_item_page(docs, kind, it, here=dirname, depth=depth)
            write_page(out / dirname / f"{s}.md", s, title, page)
            section += f"- [{label}]({dirname}/{s}.md)\n"
        index_sections.append(section)

    (out / "index.md").write_text(
        f"---\nid: {crate_slug}\ntitle: {crate_slug}\n---\n\n# {crate_slug}\n\n"
        "[Re-exports](re-exports.md)\n\n"
        + "".join(index_sections),
        encoding="utf-8",
    )

    # re-exports.md：列出经 crate 根 `pub use` 重导出的 item（含 glob 展开），按 kind 分组
    plural = {"struct": "Structs", "enum": "Enums", "trait": "Traits", "function": "Functions",
              "type_alias": "Type Aliases", "constant": "Constants", "static": "Statics"}
    rex_lines = [
        "---", f"id: {crate_slug}-re-exports", f"title: {crate_slug} Re-exports", "---", "",
        f"# {crate_slug} Re-exports", "",
        "以下 item 经 crate 根的 `pub use` 重导出（含 `*` glob 展开），完整文档见其分类目录。", "",
    ]
    any_rex = False
    for kind in ("struct", "enum", "trait", "function", "type_alias", "constant", "static"):
        items = [it for it in (found.get(kind) or []) if it["id"] in docs.reexport_ids]
        if not items:
            continue
        any_rex = True
        dirname, _prefix = KIND_DIRS[kind]
        rex_lines.append(f"## {plural[kind]}")
        rex_lines.append("")
        for it in items:
            s = slugs[it["id"]]
            label = it["name"] if s == it["name"] else s
            rex_lines.append(f"- [{label}]({dirname}/{s}.md)")
        rex_lines.append("")
    if not any_rex:
        rex_lines.append("_本 crate 无重导出。_")
    (out / "re-exports.md").write_text("\n".join(rex_lines), encoding="utf-8")

    total = sum(len(v) for v in found.values())
    print(f"[{args.crate}] 生成完毕：{total} 个 item → {out}（源 {json_p}）")
    for kind in found:
        print(f"  {kind}: {len(found[kind])}")


if __name__ == "__main__":
    sys.exit(main())
