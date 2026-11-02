#!/usr/bin/env python3
"""Build submission/technical_report.html from the manuscript Markdown.

Open the HTML in a browser and print to PDF (A4). Requires the `markdown`
package. On this machine the system python3 has it, while the project venv
does not; run:

    python3 submission/build_technical_report_html.py
"""
import pathlib
import re

import markdown

ROOT = pathlib.Path(__file__).resolve().parents[1]
SRC = ROOT / "manuscript" / "technical_report_draft.md"
OUT = ROOT / "submission" / "technical_report.html"

text = SRC.read_text(encoding="utf-8")
m = re.search(r"## 1\. Title\s*\n+\*\*(.+?)\*\*", text, re.S)
if not m:
    raise SystemExit("could not find title in manuscript")
title = m.group(1).strip()
body_md = text[text.index("## 2. Abstract"):]
body_md = body_md.replace("## 2. Abstract (~250 words)", "## Abstract", 1)
body_html = markdown.markdown(
    body_md,
    extensions=["tables", "fenced_code", "sane_lists"],
    output_format="html5",
)

html = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>
  @page {{ size: A4; margin: 18mm; }}
  body {{
    font-family: Georgia, "Times New Roman", serif;
    font-size: 11pt;
    line-height: 1.45;
    max-width: 180mm;
    margin: 0 auto;
    color: #111;
  }}
  h1 {{ font-size: 20pt; line-height: 1.2; margin: 0 0 6pt 0; }}
  h2 {{ font-size: 14pt; margin-top: 18pt; page-break-after: avoid; }}
  h3 {{ font-size: 12pt; margin-top: 14pt; page-break-after: avoid; }}
  .author {{ font-size: 12pt; margin: 4pt 0 0 0; }}
  .date {{ font-size: 10pt; color: #444; margin: 2pt 0 0 0; }}
  .disclaimer {{
    font-size: 9pt; color: #444; border-left: 3px solid #999;
    padding-left: 8pt; margin: 10pt 0 14pt 0;
  }}
  table {{ border-collapse: collapse; width: 100%; font-size: 9pt; margin: 8pt 0; }}
  th, td {{ border: 1px solid #999; padding: 3pt 4pt; vertical-align: top; }}
  th {{ background: #eee; }}
  code {{ font-family: "SFMono-Regular", Consolas, monospace; font-size: 9pt; }}
  pre {{ white-space: pre-wrap; background: #f6f6f6; padding: 8pt; }}
  blockquote {{ border-left: 3px solid #bbb; margin-left: 0; padding-left: 10pt; color: #333; }}
  hr {{ border: none; border-top: 1px solid #ccc; margin: 14pt 0; }}
  a {{ color: #0645ad; }}
</style>
</head>
<body>
<h1>{title}</h1>
<p class="author">Daniel Doyle — Independent Researcher</p>
<p class="date">Technical report draft — 2026-11-02</p>
<p class="disclaimer">
Technical report draft — an outline, not a full paper, not peer reviewed. Theoretical /
computational framework. No clinical efficacy, safety, or treatment claim. The in vitro
protocol has not been run.
</p>
{body_html}
</body>
</html>
"""
OUT.write_text(html, encoding="utf-8")
print(f"wrote {OUT} ({len(html)} bytes)")
