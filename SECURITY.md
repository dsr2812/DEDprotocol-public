# Security Policy

This is a theoretical/computational research repository: a Rust engine, a Python bridge, a CLI,
and a technical report draft.

## In scope

- Vulnerabilities in the Rust engine, CLI, or Python bridge.
- Unsafe file handling, path traversal, or parser crashes from untrusted CSV input.
- CI or repository configuration issues.
- Accidental exposure of secrets or private data in the public snapshot.

## Out of scope

- Clinical, medical-device, or biological safety claims. This project makes none.
- Patient data or personal health information. Do not send it.
- The scientific claims themselves; those belong in the issue tracker or the paper, not here.

## Reporting a vulnerability

Please use GitHub's private vulnerability reporting:

1. Open the repository's **Security** tab.
2. Choose **Report a vulnerability**.
3. Include the affected version or commit, steps to reproduce, impact, and any suggested fix.

Do **not** open a public issue for a vulnerability. Do not include patient data, personal health
information, credentials, or private keys.

## Response

Best-effort acknowledgement within 7 days. This is a small independent project; there is no
bounty. Credit is given if you want it.

## Supported versions

The latest tagged release (`v1.0.x`). Older snapshots are not supported.
