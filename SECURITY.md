# Security Policy

Beam CSS is pre-release. Please report suspected vulnerabilities privately so
we can investigate before public disclosure.

## Reporting

Email security reports to `security@beamcss.dev` with:

- affected package or crate
- reproduction steps
- expected and actual behavior
- any known exploitability details

We will acknowledge reports within 72 hours and coordinate a fix, advisory, and
release when needed.

## Supply Chain Commitments

- No package uses `postinstall` scripts.
- Release publishing should use npm provenance from GitHub Actions.
- Parser changes must include tests and fuzz coverage.
- Rust code targets `#![forbid(unsafe_code)]`.

