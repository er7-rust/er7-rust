[er7-rust](../../index.md) → [spec](../index.md) → software bills of materials

# §15 Software bills of materials

An SBOM is an inventory, not an assurance claim. It answers which Cargo
packages a particular build surface resolves and how they depend on one
another. It does not prove who performed the build, that the source was
untampered with, that the binary matches the source, or that the listed
components are vulnerability-free. Those require provenance, signatures,
and vulnerability analysis respectively.

## §15.1 Format and pinned generator

CI emits CycloneDX 1.7 JSON (`*.cdx.json`) with `cargo-cyclonedx 0.5.10`.
Both values are explicit in `bin/generate-sboms`; silently accepting whatever
format or generator happens to be installed would make two runs incomparable.
The generator is development tooling only and never becomes a dependency of
any published crate.

`SOURCE_DATE_EPOCH` is the release commit's timestamp. Version 0.5.10 omits
the random serial number when that variable is present, so repeated generation
does not differ merely because it happened later.

## §15.2 Four documents, not one workspace aggregate

The retained CI artifact contains:

1. one SBOM generated from each of the three normalized crates.io packages
   after `cargo package --locked` has assembled and verified it; and
2. one SBOM for `er7/fuzz`, which is its own Cargo workspace and is not
   published.

Generating the published SBOMs from the root workspace would be wrong:
workspace feature unification and the Criterion benchmark members can add
packages that the published crate does not resolve. Conversely, folding the
fuzz workspace into a release SBOM would imply that `libfuzzer-sys` ships to
library users. Its separate document keeps build dependencies, marked with
CycloneDX's `excluded` scope, visible to maintainers without making that claim.

## §15.3 One explicit target

The documents describe `x86_64-unknown-linux-gnu`, the hosted runner that
generates them, and carry that target triple in their metadata and filenames.
They do not claim to be target-neutral. `cargo-cyclonedx --target all` was
tested and rejected: it included target-disabled lockfile entries in the
`serde-er7` component list even though Cargo's normal dependency tree did not
select them.

`bin/check-sboms` independently asks `cargo metadata --filter-platform` for
the named target, walks normal and build dependency edges while excluding dev
edges, and requires exact agreement with each CycloneDX component list. That
check catches benchmark/dev leakage and missing transitive packages instead
of trusting file generation alone.

## §15.4 Retention and release use

The `packages` CI job uploads all four checked documents as the
`cyclonedx-sboms` run artifact. For a crate release, retain the matching
`<crate>-<version>-x86_64-unknown-linux-gnu.cdx.json` from the green run for
the release commit; the fuzz document is maintenance evidence, never a crate
release attachment. If a GitHub Release is created, attach the matching file.

GitHub's artifact digest protects the uploaded artifact while GitHub retains
it, but this project does not call that build provenance or a signed
attestation. The distinction remains explicit until a separate provenance
design exists.

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
