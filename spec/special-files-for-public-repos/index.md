# Special files for public repos

Special files that use top-level markdown:

- README.md
- LICENSE.md with SPDX license information
- CITATION.cff with ORCID citation for Joel Parker Henderson (joel@joelparkerhenderson.com) (see ~/git/assertables/assertiables/CITATION.md for template)
- NEWS.md with news, update information, press contacts, etc.
- COMPARISONS.md comparisons to relevant projects, context, etc.
- BENCHMARKS.md with any benchmarks, speed tests, optimization profiles, etc.
- INSTALL.md how to install and use any of the software
- CONTRIBUTING.md how a person can contribute their time, or update code, or donate money
- CODEOWNERS with joel@joelparkerhenderson.com
- MAINTAINERS.md with Joel Parker Henderson (joel@joelparkerhenderson.com) as sole maintainer (use this as template: https://github.com/rubentalstra/FerroEHR/blob/develop/MAINTAINERS.md)
- CHANGELOG.md with change log history summaries
- AI_STATEMENT.md (use this as template: https://github.com/rubentalstra/FerroEHR/blob/develop/AI_STATEMENT.md)
- GOVERNANCE.md how decisions are made, what binds them, how to disagree, how to become a maintainer
- SECURITY.md how to report a vulnerability, what is in scope, response windows, known open issues
- CODE_OF_CONDUCT.md Contributor Covenant 2.1, plus this project's claim-accuracy clause
- PHI.md what the software does and does not do with patient data, in plain language
- RFC.md the open questions this project wants answered, and what feedback helps
- TRADEMARKS.md the trademark notice, and what the project does not claim
- LICENSES/ the full text of every licence the SPDX expression offers (REUSE convention)
- .github/FUNDING.yml the donation routes CONTRIBUTING.md points at

This list is kept in step with the canonical version in the sibling
`fhir-rust` repository (`spec/special-files-for-public-repos/index.md`
there); re-synced 2026-08-26, re-diffed line by line 2026-08-27 and found
one wording gap (this file's CODE_OF_CONDUCT.md entry did not name the
Contributor Covenant version; fixed to match). One local addition:
TRADEMARKS.md, which this repository carries as its own top-level file
and the canonical copy does not.

## Status in this repository

All of the above exist as of 2026-08-26. Two notes:

- **The root CODE_OF_CONDUCT.md is Contributor Covenant 2.1**, matching
  what `spec/professionalization/index.md` rule 7 names, since 2026-08-27
  — it was 2.0 until then; `tasks.md` records the change and the one
  textual difference between the versions.
- **The HL7® trademark rules in
  [`spec/hl7-trademarks-fair-use/`](../hl7-trademarks-fair-use/index.md)
  are met by all of these files as of the 2026-08-26 sweep.** They require
  `®` after the first prose use of `HL7` and `FHIR` on each page, plus the
  endorsement disclaimer wherever the marks appear; `bin/check-trademarks`
  verifies the whole tree and runs in `.github/workflows/ci.yml`.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
