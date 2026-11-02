# Technical report submission package

This folder is for posting the DEDprotocol **technical report draft** and the proposed AC
impedance pre-test. It is the public posting snapshot of the `v1.0.0` engine release. This
package is documentation, not a new engine version.

**Published DOI:** **10.5281/zenodo.23897845** — <https://doi.org/10.5281/zenodo.23897845>

## Files

- `COVER_LETTER.md` — honest cover note for a lab or reader.
- `PROPOSED_EXPERIMENT.md` — one-page AC impedance pre-test (U87-MG, 8–12 cells).
- `METADATA.md` — title, author, abstract, keywords, license, subject.
- `../CITATION.cff` — machine-readable citation (repo root).
- `technical_report.html` — print this to PDF from a browser.
- `build_technical_report_html.py` — regenerates `technical_report.html` from the manuscript.
  Run it with `python3 submission/build_technical_report_html.py` (the system Python has the
  `markdown` package; the project venv may not).

## OSF Preprints (recommended first)

1. Create an account at <https://osf.io>.
2. Create a project: **DEDprotocol — technical report draft and proposed experiment**.
3. Open `technical_report.html` in a browser and print to PDF (A4, margins default).
4. Upload the PDF, `PROPOSED_EXPERIMENT.md` (or its PDF), `COVER_LETTER.md`, and the root
   `CITATION.cff`.
5. Choose **OSF Preprints** as the server, and label the item a **technical report draft**, not
   a peer-reviewed preprint.
6. Subject: **Life Sciences → Computational Biology**, or **Medicine and Health Sciences →
   Oncology** if available.
7. Paste the title, abstract, keywords, and author details from `METADATA.md`.
8. Publish. Add the DOI to `README.md`, `CITATION.cff`, and the GitHub release notes.

## Zenodo (code DOI)

1. Sign in at <https://zenodo.org>.
2. Link the GitHub repository `dsr2812/DEDprotocol-public`.
3. Create a release from tag `v1.0.0` (or upload the repo ZIP manually).
4. Get the DOI.
5. Add the DOI to the OSF project and to `CITATION.cff`.

## Optional: arXiv

`q-bio` may require an endorsement. OSF first; arXiv optional. Do not wait for arXiv to post.
arXiv expects a full paper; this is an outline, so OSF is the better first home.

## Checklist

- [ ] `technical_report.html` printed to PDF.
- [ ] Title, author, abstract match `METADATA.md`.
- [x] Email filled in `COVER_LETTER.md` and `METADATA.md`.
- [x] DOI added after publication: `10.5281/zenodo.23897845`.
- [ ] No clinical efficacy, safety, or treatment claim anywhere.
- [ ] `v1.0.0` tag points at the public snapshot commit.
- [ ] The item is labelled a technical report draft, not a peer-reviewed preprint.

## Boundary

This is a theoretical/computational technical report draft and a proposed measurement. The lab
test has not been run. It is not a treatment, a cure, or a clinical claim.
