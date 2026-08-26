[er7-rust](../../index.md) → [spec](../index.md) → promote

# §3 Promotion

How this family of crates reaches the professionals who would use it:
HL7® interface analysts, integration engineers, EHR and device vendors,
health-system platform teams, and the Rust developers those teams hire.

This is a research document and a plan, not a policy. Everything below was
checked on **2026-08-26**; links rot and community rules change, so verify
before acting on any single line. Where a claim is a judgement rather than
a fact, it says so.

## Contents

- [§3.1 Who we are actually talking to](#31-who-we-are-actually-talking-to)
- [§3.2 What we have to promote](#32-what-we-have-to-promote)
- [§3.3 Positioning: the three stories](#33-positioning-the-three-stories)
- [§3.4 Channel: the Rust community](#34-channel-the-rust-community)
- [§3.5 Channel: the HL7 and health-interop community](#35-channel-the-hl7-and-health-interop-community)
- [§3.6 Channel: trade press and reporters](#36-channel-trade-press-and-reporters)
- [§3.7 Channel: direct email outreach](#37-channel-direct-email-outreach)
- [§3.8 Channel: owned surfaces](#38-channel-owned-surfaces)
- [§3.9 Channel: conferences and events](#39-channel-conferences-and-events)
- [§3.10 A ninety-day sequence](#310-a-ninety-day-sequence)
- [§3.11 Templates](#311-templates)
- [§3.12 Etiquette, and the things that would backfire](#312-etiquette-and-the-things-that-would-backfire)
- [§3.13 Measuring whether any of it worked](#313-measuring-whether-any-of-it-worked)

## §3.1 Who we are actually talking to

Four audiences, with almost nothing in common except the message format.
A pitch written for one reads as noise to the others, so every piece of
outreach below names which audience it is for.

| Audience | Where they already are | What they care about | What makes them click |
| -------- | ---------------------- | -------------------- | --------------------- |
| **A. HL7 interface analysts and integration engineers** | Mirth/NextGen Connect discussions, InterSystems Developer Community, employer Slack, LinkedIn | Getting a feed live this week; not breaking a production interface | A CLI that shows them every value with the path that names it, and a redactor that lets them paste a real-looking message into a ticket |
| **B. Platform and infrastructure engineers at health-tech vendors** | Hacker News, r/rust, lobste.rs, docs.rs | Supply chain, audit surface, memory safety, throughput | "Zero dependencies" and a benchmark number |
| **C. Rust developers who have landed in healthcare** | This Week in Rust, r/rust, users.rust-lang.org, Rust conferences | Whether a crate is real and maintained | Spec-driven development, a visible test-to-rule mapping, an MSRV policy |
| **D. Standards, research, and public-sector people** | HL7 work groups, chat.fhir.org, INTEROPen, NHS England digital, academic informatics | Correctness against the standard; citability; licence compatibility | `CITATION.cff` with an ORCID, the five-licence choice, and the spec directory |

The single most under-served of the four is **A**. There are a lot of
people whose job is HL7 v2 all day, and almost none of the open-source
tooling aimed at them is a small fast local command-line tool. That is a
judgement, not a measurement, but it is the judgement this plan is built
on.

## §3.2 What we have to promote

Promotion works better when it has a specific artefact behind it than when
it has a project. The concrete hooks, in rough order of how well they
travel:

| Hook | Audience | Why it travels |
| ---- | -------- | --------------- |
| **`er7-redact`: de-identify an HL7 message without changing its shape** | A, B, D | It solves a problem every interface team has and most solve with a hand-rolled script: getting a message into a bug report, a test fixture, or a vendor ticket without shipping patient detail |
| **`er7` CLI: every value with the HL7 path that names it** | A | It is a demo that fits in one terminal screenshot, and the output is copy-pasteable back in as a query |
| **Zero runtime dependencies** | B, C | A `[dependencies]` table that is empty on purpose, with a test that enforces it, is unusual enough to be the headline for the Rust audience |
| **Byte-for-byte round-trip** | A, B | "What went in comes back out" is the property an interface engineer actually needs and rarely gets |
| **Spec-driven development, rules bound to tests** | C, D | The `spec/` tree, the rule IDs, and the coverage table are the credibility argument |
| **Five-licence choice** (`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only`) | D | Removes a procurement conversation before it starts |
| **MSRV of stable minus three** | A, B | Speaks directly to organisations on quarterly toolchain approvals |

Assets that support these live in the repository root and should be kept
current before any push: [`NEWS.md`](../../NEWS.md) (announcements and
press contact), [`COMPARISONS.md`](../../COMPARISONS.md),
[`BENCHMARKS.md`](../../BENCHMARKS.md), [`INSTALL.md`](../../INSTALL.md),
[`CHANGELOG.md`](../../CHANGELOG.md), and
[`AI_STATEMENT.md`](../../AI_STATEMENT.md).

## §3.3 Positioning: the three stories

One project, three true sentences. Pick by audience; never mix them in one
message.

1. **The tool story (audience A).**
   "A single small binary that reads an HL7 v2 message and tells you what
   is in every field, by path — and a second one that strips the patient
   detail out so you can paste it into a ticket."

2. **The supply-chain story (audiences B and D).**
   "HL7 v2 parsing with an empty dependency table, enforced by a test.
   Healthcare integration code gets audited; this is built to be audited."

3. **The discipline story (audiences C and D).**
   "Every behaviour is a numbered rule in a spec directory, every rule maps
   to the test that enforces it, and the spec is canonical when it and the
   code disagree."

The one thing to avoid claiming: that this is an interface engine, a
validator, or an HL7® FHIR® standard tool. It is deliberately none of
those, and the crates' own specs say so. Claiming otherwise gets a
correction from exactly the experts we most want reading.

## §3.4 Channel: the Rust community

| Venue | What it is | How to use it | Notes |
| ----- | ---------- | ------------- | ----- |
| [This Week in Rust](https://this-week-in-rust.org/) | The weekly newsletter almost every working Rust developer skims | Open a PR against [`rust-lang/this-week-in-rust`](https://github.com/rust-lang/this-week-in-rust) adding a link to the draft | **A bare link to a repo or crates.io page is explicitly discouraged.** Submit a written post — an introduction with examples, or what a release changed. Write the blog post first, submit that |
| TWiR "Crate of the Week" | Nomination thread | Nominate via the [users.rust-lang.org thread](https://users.rust-lang.org/t/crate-of-the-week/2704) or the TWiR repo | Self-nomination is accepted but weak; better if someone else does it after the blog post lands |
| [r/rust](https://www.reddit.com/r/rust/) | Large, technically hostile to marketing | Post the write-up, not the repo. Answer every comment | Check the current sidebar rules before posting; they change. The general Reddit 90/10 participation norm applies |
| [users.rust-lang.org](https://users.rust-lang.org/) | Official Rust forum, "Show and tell" category | A release thread that stays useful, not a one-off | Lower traffic, longer half-life, better tone than Reddit |
| [Hacker News](https://news.ycombinator.com/) | Show HN | "Show HN: er7 — an HL7 v2 command-line tool with zero dependencies" | Post once, in the morning US Eastern, then be present in the thread for several hours. The healthcare-integration angle is unusual enough there to be interesting on its own |
| [lobste.rs](https://lobste.rs/) | Small, high signal, invite-only to post | Only if you have an account; tag `rust` and `programming` | Do not ask strangers for an invite in order to self-promote |
| [lib.rs](https://lib.rs/) | Alternative crate index many people prefer | Nothing to do — it indexes crates.io — but check the category placement looks right | Category and keyword choice in `Cargo.toml` is the whole SEO story here |
| [Rust in Production](https://corrode.dev/podcast/) (corrode) | Bi-weekly podcast about companies running Rust | Pitch only once there is a named production user | The show is about deployments, not libraries. Premature pitch burns the contact |
| Awesome lists | [`rust-unofficial/awesome-rust`](https://github.com/rust-unofficial/awesome-rust) | PR into the relevant section | Read the inclusion criteria first; low-quality PRs are closed on sight |

**Sequencing matters here.** The write-up comes first, then Hacker News and
r/rust the same day, then the TWiR PR pointing at the write-up. Doing TWiR
first spends the one submission on a bare link.

## §3.5 Channel: the HL7 and health-interop community

This is where audience A and D actually live, and it is the channel most
open-source Rust projects never touch.

| Venue | What it is | How to use it |
| ----- | ---------- | ------------- |
| [chat.fhir.org](https://chat.fhir.org/) | HL7's Zulip. Free to join, roughly 23,000 members across 350+ streams. HL7 retired `chat.hl7.org` and consolidated here | Despite the name it is not HL7® FHIR® standard-only: there is an active **`v2 to fhir`** stream, and v2 questions get answered. Join, participate for weeks before posting anything of your own, then post `er7-redact` in the stream where de-identification is on topic. Read the [community expectations](https://confluence.hl7.org/display/FHIR/Chat.fhir.org+Community+Expectations) first — they are enforced |
| [community.fhir.org](http://community.fhir.org/) | Forum, ~1,500 members, longer-form | Better than Zulip for a post that should still be findable in a year |
| [NextGen Connect / Mirth discussions](https://github.com/nextgenhealthcare/connect/discussions) | Where the Mirth community now discusses, having moved from [forums.mirthproject.io](https://forums.mirthproject.io/) | The single highest-density concentration of audience A anywhere. Answer real HL7 questions there; mention the tool only when it is the actual answer |
| [InterSystems Developer Community](https://community.intersystems.com/tags/hl7) | Vendor-run but genuinely active on HL7 v2 | Same posture: answer first, link second |
| [HL7 International work groups](https://www.hl7.org/) | The standards bodies themselves | Membership costs money; the [InM](https://www.hl7.org/Special/committees/inm/) and v2 management groups are where v2's future is decided. Worth it for audience D credibility, not for downloads |
| [HL7 UK](https://uk.linkedin.com/company/hl7-uk) | The UK affiliate | Given the NHS work in adjacent repositories, this is the closest thing to a home audience |
| [INTEROPen](https://www.interopen.org/) | UK open collaboration of suppliers, providers, and standards people | UK-specific; pairs with HL7 UK. Site was under reconstruction when checked — verify it is live before pointing anyone at it |
| LinkedIn | [HL7 International](https://www.linkedin.com/company/health-level-seven), plus interface-engineer groups | The one social network where audience A is reliably present in a professional posture. A short post with one terminal screenshot outperforms a link |
| Awesome lists | [`kakoni/awesome-healthcare`](https://github.com/kakoni/awesome-healthcare), [`fhir-fuel/awesome-FHIR`](https://github.com/fhir-fuel/awesome-FHIR), the various `awesome-health` forks | A PR to `awesome-healthcare` is cheap and durable. Pick the fork that is actually maintained — several are stale |

**The rule that governs this whole section:** these are professional
communities where reputation is the currency and people remember who shows
up only to advertise. Budget weeks of genuine participation before the
first mention of your own work. This is slower than the Rust channels and
worth more.

## §3.6 Channel: trade press and reporters

Realistic expectation first: a single-maintainer open-source library is not
a story for a healthcare trade publication on its own. It becomes a story
when it is attached to something else — an adoption, a de-identification
angle, a security or supply-chain angle, or a named health system using it.

| Outlet | Beat | Angle that could work |
| ------ | ---- | --------------------- |
| [Healthcare IT News](https://www.healthcareitnews.com/topics/hl7) | HIMSS-owned; runs an active HL7/interoperability topic page | De-identification tooling; open source in interface engineering |
| [HISTalk](https://histalk2.com/) | The industry's insider newsletter; reads everything, links generously | A short, specific note to the tips address is far more likely to land than a formal release |
| [Digital Health (UK)](https://www.digitalhealth.net/) | UK NHS digital beat | NHS/HL7 UK angle, open-source-in-the-NHS angle |
| [Healthcare Innovation](https://www.hcinnovationgroup.com/), [Fierce Healthcare](https://www.fiercehealthcare.com/) | US health IT | Only with an adoption story |
| [The Standard](https://blog.hl7.org/) — HL7's own blog | HL7's official blog | Contributed posts happen; ask the HL7 comms team |
| [InfoQ](https://www.infoq.com/), [The New Stack](https://thenewstack.io/) | Developer press | The Rust-in-healthcare angle, which is genuinely novel |
| [Association of Health Care Journalists](https://healthjournalism.org/) | Not an outlet — the professional body | Useful for [finding who covers what](https://healthjournalism.org/helpful-links/industry-groups/); do not pitch the association itself |

**How to pitch, in order of what actually works:**

1. Be a useful source before you are a subject. Reporters covering HL7 keep
   a short list of people who explain things clearly on deadline. Offer
   that, with no ask attached.
2. Pitch a story, not a product. "A tool that lets hospital interface teams
   share real-shaped HL7 messages without sharing patient data" is a story.
   "er7-redact 0.2.0 is released" is not.
3. Keep it to five sentences with the link at the bottom.
4. Never attach a PDF press release.

`NEWS.md` in the repository root exists so a reporter who lands on the
GitHub page finds a press contact and a factual summary without having to
ask.

## §3.7 Channel: direct email outreach

The highest-conversion channel and the one most likely to be done badly.
Small numbers, hand-written, one at a time.

| Target | Why them | The ask |
| ------ | -------- | ------- |
| Maintainers of adjacent open-source HL7 tooling (Mirth/Connect, [hl7apy](https://github.com/crs4/hl7apy), [HAPI](https://hapifhir.github.io/hapi-hl7v2/)) | They know the problem space and their users are audience A | Not a favour — a genuine "here is a thing that complements yours; would a link be useful to your users?" |
| Authors of Rust HL7 crates ([`hl7-parser`](https://github.com/hamaluik/hl7-parser), [`hl7v2-parser`](https://github.com/EffortlessMetrics/hl7v2-rs), [`rust-hl7`](https://github.com/wokket/rust-hl7)) | A tiny field where cooperation beats competition | Compare notes; cross-link where honest. See [`COMPARISONS.md`](../../COMPARISONS.md) |
| NHS trust and health-system integration leads | Audience A with budget | Only through an existing relationship. Cold email to clinical organisations does not work |
| Academic health informatics groups | Cite-and-teach audience | The `CITATION.cff` and the spec directory are the pitch |
| Health-tech vendors with public engineering blogs | They write about their stack | Offer the "why zero dependencies" argument as a guest post |

Rules: one person, one email, no template that looks like a template, no
follow-up more than once, and no mailing list built from scraped addresses.
The sender address is `joel@joelparkerhenderson.com` and it should stay a
person, not a `noreply@`.

## §3.8 Channel: owned surfaces

These cost nothing per push and compound. They should be right before any
outbound campaign, because every channel above sends traffic into them.

| Surface | State | What to do |
| ------- | ----- | ---------- |
| <https://er7-rust.github.io/> | Live; source in `er7-rust.github.io/` in this repository | Add a page per story in §3.3. A landing page that answers "what is this, in one screen" is the single highest-leverage asset |
| [docs.rs](https://docs.rs/er7/) | Generated per release | The crate-level doc comment is the first thing many people read. Treat it as the pitch, not the API index |
| [crates.io](https://crates.io/crates/er7) | Live | `description`, `keywords`, and `categories` are the only search surface. Keywords are capped at five: `hl7`, `er7`, `healthcare`, `parser`, `pipe-delimited` today — revisit when the field changes |
| GitHub repository | Live | Topics, a description, and a social preview image. Pinned repositories on the `er7-rust` org page |
| `NEWS.md` | Added alongside this document | The canonical announcement history and the press contact |
| A release blog post per minor version | Not started | This is what feeds This Week in Rust. Without it there is nothing to submit |

## §3.9 Channel: conferences and events

| Event | When | Fit |
| ----- | ---- | --- |
| [HL7 Working Group Meetings and Connectathons](https://www.hl7.org/events/) | Three per year. 40th Annual Plenary + WGM + FHIR Connectathon Jan 2027; WGM Denver May 2027; 41st Annual Plenary Dallas Sept 2027 (verify on the events page — dates move) | The best room in the world for audience A and D. Connectathons in particular reward showing up with a working tool |
| [FHIR DevDays](https://www.devdays.com/) | Annual, US and EU editions | Centred on the HL7® FHIR® standard, but the v2-to-FHIR® migration track is exactly our territory |
| [HIMSS](https://www.himss.org/) | Annual, March | Enormous, expensive, and aimed at buyers. Skip unless someone else is paying |
| [RustConf](https://rustconf.com/) | Sept 2026 Montréal + online; CFP via Sessionize | "Rust in a regulated healthcare pipeline" is a talk that does not exist yet |
| [EuroRust](https://eurorust.eu/), [RustNL](https://rustnl.org/), [Rust Nation UK](https://www.rustnationuk.com/) | Various | Rust Nation UK is the natural fit given the NHS angle. Watch [confs.tech/rust](https://confs.tech/rust) and [corrode.dev's conference list](https://corrode.dev/blog/rust-conferences-2026/) for open CFPs |

A conference talk is the highest-effort item on this page and the one that
generates the most durable credibility. One good talk outperforms a year of
posting.

## §3.10 A ninety-day sequence

Ordered so that each step has something to point at by the time it runs.
Weeks are relative to whenever this starts, not to any calendar date.

| Weeks | Do | Audience |
| ----- | -- | -------- |
| 1–2 | Get the owned surfaces right: landing page, crate-level docs, `NEWS.md`, `COMPARISONS.md`, `BENCHMARKS.md`, `INSTALL.md`. Nothing outbound | — |
| 1–8 | Join chat.fhir.org, the Mirth discussions, and the InterSystems community. Answer other people's HL7 questions. Mention nothing of your own | A, D |
| 3 | Write the first real post: "Showing every value in an HL7 v2 message, by path" — a tool post, with terminal output | A, B |
| 4 | Show HN and r/rust the same day, on the post. Be in both threads all day | B, C |
| 5 | TWiR PR pointing at the post. PR to `awesome-healthcare` | C |
| 6 | Write the second post: "De-identifying an HL7 message without changing its shape". This is the one with the widest reach | A, B, D |
| 7 | Post the second piece where de-identification is on topic in the HL7 communities — now that you have eight weeks of participation behind you. LinkedIn post with one screenshot | A, D |
| 8 | Email the adjacent maintainers, one at a time | — |
| 9 | Write the third post: "Why this crate's dependency table is empty, and the test that keeps it that way" | B, C |
| 10 | Pitch HISTalk and one trade outlet on the de-identification angle only | Press |
| 11–12 | Submit a conference proposal to whichever Rust or HL7 CFP is open. Review what worked against §3.13 | — |

## §3.11 Templates

Starting points, to be rewritten every time. A template that is sent as
written reads as one.

### This Week in Rust submission

> **er7 0.2.0: showing every value in an HL7 v2 message, by path**
> <link to the post>

Nothing more. The PR adds one line to the draft's "Project/Tooling
Updates" section. The link must go to a written post, not to the repository
or the crates.io page.

### Show HN

> **Show HN: er7 — an HL7 v2 command-line tool with zero dependencies**
>
> HL7 v2 is the message format most hospital systems still talk in, and its
> meaning is entirely positional — one misplaced `|` silently shifts
> everything after it. I wanted a small local tool that shows every value
> with the path that names it, and that round-trips byte for byte.
>
> It has an empty `[dependencies]` table, on purpose, with a test that
> fails if anything is added: this is code meant to sit at the bottom of a
> stack somebody has to audit.
>
> There is a sibling that redacts patient detail without changing the shape
> of the message, which is the part I actually use most.

### First post in an HL7 community

> I have been working on a small command-line tool for HL7 v2 in ER7, and
> the piece that seems most useful to other people here is the redactor: it
> replaces patient detail but keeps the message's shape, delimiters, and
> escape sequences intact, so a redacted message still exercises the same
> parser path as the original. That makes it safe to attach to a ticket.
>
> <link>. It is open source under a choice of five licences. I would
> genuinely like to know where it gets HL7 wrong — the specification it is
> built from is in the repository, and corrections from this group would
> improve it.

### Reporter pitch

> Subject: hospital interface teams and the HL7 messages they cannot share
>
> Hi <name> — I read your piece on <specific thing>.
>
> There is a small, unglamorous problem in hospital integration work:
> when an HL7 interface breaks, the message that broke it contains patient
> data, so the engineer debugging it cannot paste it into a ticket, a test
> suite, or a vendor support case. Most teams solve it with a hand-rolled
> script that quietly changes the message's shape and stops reproducing the
> bug.
>
> I have built an open-source tool that redacts the detail while preserving
> the structure exactly. Happy to explain the problem on background whether
> or not you write about the tool.
>
> Joel Parker Henderson — <joel@joelparkerhenderson.com>

### Maintainer-to-maintainer email

> Subject: er7 (Rust HL7 v2) — cross-linking?
>
> Hi <name> — I maintain `er7`, a Rust HL7 v2 library and CLI. Yours was
> one of the projects I read before starting, and the write-up in
> `COMPARISONS.md` says plainly where yours is the better choice: <one true
> specific example>.
>
> If a link back would be useful to your users I would be glad to add one
> in either direction, and no hard feelings if not.

## §3.12 Etiquette, and the things that would backfire

These are hard rules, not preferences. This is healthcare; the audience is
professional and small, and a single misstep is durable.

1. **Never post a real patient message.** Every example, screenshot, demo,
   sample file, and benchmark input is synthetic. This is already family
   policy ([§1.4](../01-family-policy/index.md)) and it applies to
   promotion without exception. A redacted real message is still a real
   message.
2. **Do not claim regulatory status.** Not HIPAA-compliant, not certified,
   not validated, not a medical device. The software is a library; the
   compliance obligation stays with the deployer.
3. **Do not overstate the standard's coverage.** The crates do what their
   specs say and no more. An HL7 audience will check.
4. **No sock puppets, no vote rings, no upvote requests**, on any platform.
5. **Disclose the AI-assisted development** where it is relevant — see
   [`AI_STATEMENT.md`](../../AI_STATEMENT.md). Some communities ask; being
   the one who said it first is much better than being the one who was
   found out.
6. **Do not pitch inside somebody else's support thread.** Answer the
   question. If the tool is the answer, say so once, plainly.
7. **State the bus factor honestly.** [`MAINTAINERS.md`](../../MAINTAINERS.md)
   says there is one maintainer. Anyone considering this for a clinical
   pipeline deserves that up front, and saying it builds more trust than
   hiding it.
8. **No scraped mailing lists, ever.** Beyond the etiquette, GDPR and CAN-SPAM
   are real, and health-sector recipients are exactly the population that
   reports it.

## §3.13 Measuring whether any of it worked

Downloads are the obvious metric and the least informative one; a TWiR
mention produces a spike of CI machines. Better signals, in order of value:

| Signal | Why it matters | Where to see it |
| ------ | -------------- | ---------------- |
| An issue or PR from someone who is clearly an interface engineer | Audience A arrived, and stayed long enough to care | GitHub |
| A question that shows they read the spec | The credibility story landed | GitHub, Zulip |
| Another project depending on the crate | The only durable adoption signal | [crates.io reverse dependencies](https://crates.io/crates/er7/reverse_dependencies) |
| A mention by someone who is not you | Word of mouth started | Search, GitHub mentions |
| Sustained recent-download baseline between releases | Real use, not a spike | crates.io recent downloads |
| Total downloads | Weakest signal; record it anyway | crates.io |

Record the baseline in `NEWS.md` at each release so the before-and-after of
a campaign is legible later. As of 2026-08-26 the baseline is: `er7` 161
total downloads, `er7-redact` 64, `serde-er7` 48 — all three published
within the previous two weeks.

## Sources

Checked 2026-08-26.

- [This Week in Rust submission guidelines](https://github.com/rust-lang/this-week-in-rust) and the [Crate of the Week thread](https://users.rust-lang.org/t/crate-of-the-week/2704)
- [chat.fhir.org community expectations](https://confluence.hl7.org/display/FHIR/Chat.fhir.org+Community+Expectations); [HL7 Zulip consolidation notice](https://chat.hl7.org/)
- [Top communities in digital health](https://www.health-samurai.io/articles/top-communities-in-digital-health) — community sizes for chat.fhir.org and community.fhir.org
- [NextGen Connect discussions](https://github.com/nextgenhealthcare/connect/discussions) and the [legacy Mirth forums](https://forums.mirthproject.io/)
- [InterSystems Developer Community, HL7 tag](https://community.intersystems.com/tags/hl7)
- [HL7 events and Work Group Meetings](https://www.hl7.org/events/workgroupmeetings.cfm); [FHIR Connectathons](https://www.hl7.org/events/fhir-connectathon/index.cfm)
- [HL7 UK](https://uk.linkedin.com/company/hl7-uk); [INTEROPen](https://www.interopen.org/)
- [Healthcare IT News HL7 topic page](https://www.healthcareitnews.com/topics/hl7); [The Standard, HL7's blog](https://blog.hl7.org/)
- [Association of Health Care Journalists industry groups list](https://healthjournalism.org/helpful-links/industry-groups/)
- [awesome-healthcare](https://github.com/kakoni/awesome-healthcare); [awesome-FHIR](https://github.com/fhir-fuel/awesome-FHIR)
- [RustConf 2026](https://rustconf.com/); [Rust conferences 2026, corrode](https://corrode.dev/blog/rust-conferences-2026/); [confs.tech/rust](https://confs.tech/rust)
- [Rust in Production podcast](https://corrode.dev/podcast/)
- crates.io API, 2026-08-26, for the download baselines and the comparison figures in [`COMPARISONS.md`](../../COMPARISONS.md)

---

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
