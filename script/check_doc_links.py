"""Validate relative markdown links in a generated docs tree.

Usage: check_doc_links.py <docs-root> <repo-root>

A link is 'broken' iff the relative target (resolving ../ segments) does not
exist on disk. Both cross-item .md links and 'Defined in' source .rs links are
checked.
"""
import re
import sys
from pathlib import Path

docs_root = Path(sys.argv[1])
# (docs_root, repo_root) both given; links are relative to the .md's own dir
# so repo_root is only informational.

LINK_RE = re.compile(r"\]\(((?:\.\./)+)?([A-Za-z0-9_./-]+\.(?:md|rs))(#[^)]*)?\)")

total = 0
broken_md = 0
broken_src = 0
for md in sorted(docs_root.rglob("*.md")):
    text = md.read_text(encoding="utf-8")
    for m in LINK_RE.finditer(text):
        rel = (m.group(1) or "") + m.group(2)
        total += 1
        target = (md.parent / rel).resolve()
        if target.suffix == ".md":
            if not target.exists():
                broken_md += 1
                if broken_md <= 8:
                    print("BROKEN md :", md.relative_to(docs_root), "->", rel)
        else:
            if not target.exists():
                broken_src += 1
                if broken_src <= 8:
                    print("BROKEN src:", md.relative_to(docs_root), "->", rel)

print(f"total={total}  broken_md={broken_md}  broken_src={broken_src}")
