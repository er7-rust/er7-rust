**Workspace review, 2026-11-19.** This archived assessment includes a
proposal to collect redacted real HL7® messages. The workspace requires
synthetic fixtures for every test. The current plan uses a shared
multilingual fixture written from scratch. See the
[family policy](../../../spec/01-family-policy/index.md) and
[reviewed tasks](../../../tasks.md).

Based on my analysis of the `er7-rust` workspace, here are **prioritized, actionable quality improvement tasks** for AI to execute, organized by category. The project is exceptionally well-documented with all professionalization prerequisites met, so these focus on **growth, sustainability, and technical depth**.

---

## 🎯 **PROMOTION & COMMUNITY BUILDING** *(Help new visitors, increase adoption)*

### **High-Impact Outreach Tasks**
| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Execute the 90-day promotion sequence from `spec/promote/§3.10` | Write 3 blog posts (tool demo, redaction, zero-deps), submit to TWiR/HN/r/rust, engage HL7 communities after 8 weeks of participation | 3-4 weeks | ★★★★★ |
| **HIGH** | Create `er7` crate of the week nomination | Draft nomination for `users.rust-lang.org/t/crate-of-the-week/2704` with compelling examples | 1-2 hours | ★★★★☆ |
| **HIGH** | Submit to awesome lists | PR to `rust-unofficial/awesome-rust`, `kakoni/awesome-healthcare`, `fhir-fuel/awesome-FHIR` | 2-3 hours | ★★★★☆ |
| **MEDIUM** | Join and participate in HL7 communities | Answer HL7 questions on `chat.fhir.org` (v2 streams), Mirth discussions, InterSystems community for 8 weeks before mentioning the project | Ongoing | ★★★★☆ |
| **MEDIUM** | Create LinkedIn showcase post | Single terminal screenshot showing `er7` output with HL7 path notation, tagged with #HL7 #Rust #HealthIT | 1 hour | ★★★☆☆ |

### **Documentation & Onboarding Improvements**
| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Add interactive tutorial to website | Step-by-step guide: "Parse your first HL7 message in 5 minutes" with embeddable code | 1-2 days | ★★★★★ |
| **HIGH** | Create comparison video/demo | Side-by-side: `er7` vs `hl7-parser` vs `HAPI` showing speed, accuracy, zero-deps advantage | 1 day | ★★★★☆ |
| **MEDIUM** | Add "First 10 Minutes" guide | Quick start for each audience: interface engineers, Rust devs, platform teams | 1 day | ★★★★☆ |
| **MEDIUM** | Create decision tree | "Which crate do I need?" flowchart in `README.md` or website | 4 hours | ★★★☆☆ |
| **MEDIUM** | Add user testimonials section | Collect and display quotes from early adopters (need to find actual users first) | 2 hours | ★★★☆☆ |
| **LOW** | Add FAQ for common HL7 patterns | "How do I handle repeating fields?" "What about custom delimiters?" etc. | 1 day | ★★☆☆☆ |

### **Upskilling & Education**
| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Create HL7 v2 primer for Rust devs | Explain ER7 encoding, pipe-hat notation, escape sequences for Rust audience | 2-3 days | ★★★★★ |
| **MEDIUM** | Record screencast series | 5-minute videos: parsing, querying, redaction, performance benchmarking | 2-3 days | ★★★★☆ |
| **MEDIUM** | Add Rust-specific HL7 glossary | Map HL7 terms (MSH, PID, OBX, segments, fields, components) to code concepts | 1 day | ★★★☆☆ |
| **LOW** | Create "HL7 for Rust Developers" workshop | Materials for Rust meetups: slides, exercises, sample messages | 3-4 days | ★★★☆☆ |

---

## 🔧 **ENGINEERING & CODE QUALITY** *(Strengthen the technical foundation)*

### **Core Engineering Tasks**
| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Add SBOM (Software Bill of Materials) | Generate SPDX or CycloneDX SBOM for supply chain transparency | 1 day | ★★★★★ |
| **HIGH** | Implement Trusted Publishing | Set up OIDC-based publishing to crates.io via GitHub Actions | 2 days | ★★★★★ |
| **HIGH** | Add second security responder | Recruit and document a backup maintainer for security issues (per `MAINTAINERS.md` gap) | Ongoing | ★★★★★ |
| **MEDIUM** | Add `serde` feature to `er7` crate | Optional `serde` support with `serde-er7` integration path | 1-2 days | ★★★★☆ |
| **MEDIUM** | Improve error types | More granular error variants for better debugging (currently very coarse) | 2-3 days | ★★★★☆ |
| **MEDIUM** | Add streaming parser | For large message files (100MB+ HL7 batches) without full loading | 3-4 days | ★★★★☆ |
| **LOW** | Add `no_std` support | Enable parsing in embedded/constraint environments | 5+ days | ★★★☆☆ |

### **Testing & Verification**
| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Expand fuzz testing | Add targets for: invalid escape sequences, malformed MSH, edge cases in repeating fields | 2-3 days | ★★★★★ |
| **HIGH** | Create property-based tests | Use `proptest` to generate valid/invalid HL7 messages and verify properties (round-trip, path consistency) | 3-4 days | ★★★★★ |
| **HIGH** | Add real-world message corpus | Collect 100+ redacted real HL7 messages from different vendors/systems for regression testing | Ongoing | ★★★★★ |
| **MEDIUM** | Add integration tests | Test full workflows: parse → query → edit → serialize → verify round-trip | 1-2 days | ★★★★☆ |
| **MEDIUM** | Add benchmark comparisons | Compare `er7` against `hl7-parser`, `HAPI` on same message corpus | 2-3 days | ★★★★☆ |
| **MEDIUM** | Add snapshot testing | For CLI output to catch unintended formatting changes | 1 day | ★★★☆☆ |
| **LOW** | Add mutation testing | Use `cargo-mutants` to verify test suite quality | 2-3 days | ★★☆☆☆ |

### **Code Health & Maintainability**
| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Refactor CLI argument parsing | Replace current approach with `clap` or `argh` for better UX | 1-2 days | ★★★★☆ |
| **HIGH** | Add comprehensive logging | Debug/trace logging for parsing decisions (opt-in via feature flag) | 2-3 days | ★★★★☆ |
| **MEDIUM** | Improve documentation coverage | Add doc tests for all public API examples | 2-3 days | ★★★★☆ |
| **MEDIUM** | Add architecture decision records (ADRs) | Document key design choices (zero-deps, round-trip guarantee, etc.) | 1 day | ★★★★☆ |
| **MEDIUM** | Add cargo features for optional functionality | `std`, `serde`, `alloc` features for different use cases | 2-3 days | ★★★☆☆ |
| **LOW** | Add internal consistency checks | Runtime assertions for invariants (valid path syntax, delimiter consistency) | 1-2 days | ★★★☆☆ |
| **LOW** | Add perf flamegraph support | For easy performance profiling in production | 1 day | ★★☆☆☆ |

---

## 🔍 **AUDITING & SECURITY** *(Hardening for production use)*

### **Security Auditing Tasks**
| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Perform security audit of parsing logic | Review for buffer overreads, allocation attacks, DoS vectors in malformed input | 3-5 days | ★★★★★ |
| **HIGH** | Add security focused fuzzing | Target: memory safety, resource exhaustion, panic resistance | 2-3 days | ★★★★★ |
| **HIGH** | Document security properties | Formal specification of what `er7-redact` does/doesn't guarantee (HIPAA Safe Harbor coverage) | 2-3 days | ★★★★★ |
| **MEDIUM** | Add memory safety proofs | Use tools like `cargo-geiger` to verify no unsafe code paths | 1 day | ★★★★☆ |
| **MEDIUM** | Add rate limiting to CLI | Prevent accidental DoS via extremely large inputs | 1 day | ★★★★☆ |
| **MEDIUM** | Review dependency vulnerabilities | Audit `libfuzzer-sys`, `criterion` for known CVEs | 1 day | ★★★☆☆ |
| **LOW** | Add security.md to each crate | Crate-specific security posture documentation | 1 day | ★★★☆☆ |

### **Compliance & Governance**
| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Add SOC2 Type II readiness checklist | Document what would be needed for enterprise compliance | 2-3 days | ★★★★☆ |
| **HIGH** | Create HIPAA compliance whitepaper | Document how to use `er7-redact` in HIPAA-compliant workflows | 2-3 days | ★★★★☆ |
| **MEDIUM** | Add license compatibility matrix | Document how the 5-license choice interacts with other open source projects | 1 day | ★★★☆☆ |
| **MEDIUM** | Review trademark usage in all published content | Audit crates.io, docs.rs, website for trademark compliance | 1 day | ★★★☆☆ |
| **LOW** | Add legal FAQ | "Can I use this in a commercial product?" etc. | 1 day | ★★☆☆☆ |

---

## 📊 **DATA & ANALYTICS** *(Measure and improve)*

| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Set up download analytics | Track crates.io downloads by version, identify adoption patterns | 1 day | ★★★★☆ |
| **MEDIUM** | Add website analytics | Track visitor paths, popular pages, drop-off points | 1 day | ★★★☆☆ |
| **MEDIUM** | Create adoption dashboard | GitHub stars, forks, issues, PRs, dependency graph | 1 day | ★★★☆☆ |
| **LOW** | Add error reporting telemetry (opt-in) | Collect anonymized parsing failure statistics to identify common HL7 patterns | 2-3 days | ★★☆☆☆ |

---

## 🎨 **WEBSITE & DOCUMENTATION ENHANCEMENTS**

| Priority | Task | Description | Effort | Impact |
|----------|------|-------------|--------|--------|
| **HIGH** | Add live demo to website | In-browser HL7 parser (WASM build of `er7` CLI) | 2-3 days | ★★★★★ |
| **HIGH** | Add search functionality | Search across all documentation (specs, docs, website) | 1-2 days | ★★★★☆ |
| **MEDIUM** | Add dark mode | For better readability | 1 day | ★★★☆☆ |
| **MEDIUM** | Create API documentation hub | Unified view of all three crates' APIs | 1-2 days | ★★★☆☆ |
| **MEDIUM** | Add interactive examples | Run code examples directly from documentation | 2-3 days | ★★★☆☆ |
| **LOW** | Add testimonial collection system | Form for users to submit feedback/quotes | 1 day | ★★☆☆☆ |

---

## 🚀 **ROADMAP ITEMS** *(From specs - already identified but need execution)*

From `er7/spec/17-open-tasks/` and similar:
- **D24**: Add custom policy support to `er7-redact` (allow users to define their own redaction rules)
- **D25**: Add policy testing framework (verify redaction policies work as expected)
- **S10**: Add `Deserialize` for `Message` with configurable parsing options
- **R26**: Add support for custom delimiters (beyond `|^~\&`)
- **R27**: Add support for non-standard escape sequences
- **All crates**: Add more comprehensive path query examples

---

## ⚡ **QUICK WINS** *(Can be done in <2 hours each)*

1. **Add missing examples** to `er7/examples/` directory
2. **Fix typos** in documentation (run `codespell` or similar)
3. **Update CHANGELOG.md** with latest releases if behind
4. **Add GitHub topic tags** to repository (hl7, er7, healthcare, rust, parsing)
5. **Create Twitter/X account** for project updates
6. **Add Discord/Slack community invite** link
7. **Update crate keywords** on crates.io for better discoverability
8. **Add badges** to READMEs (CI status, docs.rs, crates.io)
9. **Create GitHub Discussion categories** (Q&A, Ideas, Show and Tell)
10. **Add "Good First Issue" labels** to beginner-friendly tasks

---

## 📈 **SUCCESS METRICS** *(How to measure impact)*

| Category | Metric | Target |
|----------|--------|--------|
| **Adoption** | crates.io downloads | +100% in 3 months |
| **Community** | GitHub stars | +200 in 6 months |
| **Community** | GitHub forks | +50 in 6 months |
| **Community** | Discord/Slack members | 100+ active |
| **Community** | Issue/PR contributors | 10+ unique |
| **Quality** | Test coverage | >95% |
| **Quality** | CI stability | 100% green for 30 days |
| **Security** | Dependabot alerts | 0 open |
| **Security** | Security audit completion | ✅ Done |
| **Documentation** | Docs.rs completeness | All public items documented |

---

## 🎯 **RECOMMENDED STARTING POINT**

**For immediate impact, start with:**
1. Execute the 90-day promotion sequence (3 blog posts + outreach)
2. Add SBOM generation
3. Set up Trusted Publishing
4. Expand fuzz testing with real-world patterns
5. Create interactive tutorial for website

**These 5 tasks** would:
- ✅ Unlock promotion (already gated as ready)
- ✅ Improve supply chain security
- ✅ Increase test coverage against real threats
- ✅ Make the project more discoverable
- ✅ Help new users get started faster

---

Would you like me to **dive deep into any specific category** and provide detailed implementation plans, or shall I **start executing** on one of these high-priority items?

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
