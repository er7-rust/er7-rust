//! # serde-er7
//!
//! **[website](https://er7-rust.github.io/serde-er7/)**
//! •
//! **[documentation](https://docs.rs/serde-er7/)**
//! •
//! **[source](https://github.com/er7-rust/er7-rust/tree/main/serde-er7)**
//! •
//! **[crate](https://crates.io/crates/serde-er7)**
//! •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! Serde support for [`er7`], the pipe-hat encoding that carries HL7 v2
//! messages between healthcare systems.
//!
//! [`er7`] parses, queries, edits, and writes ER7 text, deliberately with
//! no dependencies of its own. This crate is the bridge from that value
//! tree to the rest of the Serde ecosystem: wrap a parsed [`er7::Message`]
//! in [`Message`], and it can flow through `serde_json`, `serde_yaml`,
//! `bincode`, or any other Serde data format — for storing a parsed
//! message in a document database, logging it as structured JSON,
//! returning it from a web API, or testing it with `assert_eq!` against a
//! literal.
//!
//! Every impl in this crate is written by hand against the low-level
//! `Serializer`/`Deserializer`/`Visitor` traits, following the pattern
//! [serde's own documentation](https://docs.rs/serde/latest/serde/)
//! walks through for a manual implementation — no `#[derive(Serialize)]`
//! anywhere, because [`er7`]'s tree cannot derive its way to the shapes
//! below (a bare array for a field's repetitions, a bare string for a
//! leaf) without help.
//!
//! Example:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_er7::Message;
//!
//! let text = "MSH|^~\\&|LAB|ACME|EHR|CLINIC|20260815120000||ORU^R01|MSG9|P|2.5\r\
//!             PID|1||12345^^^ACME^MR||SMITH^JOHN^Q||19800101|M\r\
//!             OBX|1|NM|2093-3^Cholesterol^LN||187|mg/dL|||||F";
//!
//! let message = Message::parse(text)?;
//!
//! // Out to JSON, and back — this crate never mentions JSON itself; any
//! // Serde format works the same way.
//! let json = serde_json::to_string(&message)?;
//! let back: Message = serde_json::from_str(&json)?;
//!
//! assert_eq!(back.to_er7(), text);
//! # Ok(())
//! # }
//! ```
//!
//! # The shape each level serializes as
//!
//! | Level | Wrapper | Serializes as |
//! |-------|---------|----------------|
//! | Message | [`Message`] | object: `{"separators": ..., "segments": [...]}` |
//! | Segment | [`Segment`] | object: `{"name": "PID", "fields": [...]}` |
//! | Field | [`Field`] | array of repetitions |
//! | Repetition | [`Repetition`] | array of components |
//! | Component | [`Component`] | array of subcomponent strings |
//! | Subcomponent | [`Subcomponent`] | a bare string, `raw` (not [`er7::Subcomponent::value`]-decoded) |
//! | Separators | [`Separators`] | object of six named fields, chars as one-character strings |
//! | Terminator | [`Terminator`] | one of the strings `"Cr"`, `"Lf"`, `"CrLf"` |
//!
//! Every level below [`Message`] is optional to reach for directly — most
//! callers only ever construct a [`Message`] and let its `Serialize` impl
//! walk the rest of the tree. The lower levels are `pub` so a caller who
//! only wants to serialize one segment, or one field, out of a larger
//! message can do that too.
//!
//! # What is deliberately not here
//!
//! This crate adds exactly one thing to [`er7`]: Serde support for its
//! existing value tree. It does not add a dictionary, a validator, or a
//! transport — [`er7`] does not have those either, and a bridge crate is
//! the wrong place to add what the crate it bridges declined to have. See
//! [`er7`]'s own documentation for what "encoding, not dictionary" means in
//! practice.
//!
//! It also does not pick a wire format. `serde_json` appears only as a
//! dev-dependency, to test and demonstrate against — the whole point of
//! building against `serde`'s traits rather than writing an ER7-to-JSON
//! converter directly is that the format is the caller's choice, not this
//! crate's.
//!
//! # Documentation
//!
//! `spec/index.md` in the repository is this crate's specification —
//! what each wrapper type must serialize as and why, following the same
//! spec-driven process [`er7`] itself uses. A tutorial is in
//! `docs/usage/index.md`, and runnable programs are in `examples/`.

#![warn(missing_docs)]

mod component;
mod field;
mod message;
mod repetition;
mod segment;
mod separators;
mod subcomponent;
mod terminator;

pub use component::Component;
pub use field::Field;
pub use message::Message;
pub use repetition::Repetition;
pub use segment::Segment;
pub use separators::Separators;
pub use subcomponent::Subcomponent;
pub use terminator::Terminator;

// Re-exported so a caller can name `er7::Message`, `er7::Error`, and the
// rest without adding their own dependency on `er7` — the same convenience
// `hl7-2-5-to-xml-using-rust` and `hl7-2-5-to-json-using-rust` extend for
// the crates they build on top of.
pub use er7;
