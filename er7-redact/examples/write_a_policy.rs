//! Three ways to say what to redact: build a policy in Rust, read one
//! from a file, or start from a built-in and edit it.
//!
//! Run with: `cargo run --example write_a_policy`
#![forbid(unsafe_code)]

use er7_redact::{Action, Policy, Posture, Redactor, Unrecognised};

fn main() -> Result<(), er7_redact::Error> {
    // 1. Built in Rust, rule by rule. Order matters: rules apply in the
    //    order they are listed. `accept_all` is the starting point that
    //    redacts nothing until a rule says so — the other one is
    //    `reject_all`, which redacts everything until a `keep` rule says
    //    otherwise.
    let built = Policy::accept_all()
        .with("PID-3.1", Action::Pseudonym)?
        .with("PID-5", Action::redacted())?
        .with("PID-7", Action::First(4))?
        .with("PID-19", Action::Clear)?;

    // 2. Read from a policy file — the same thing, in the form a team
    //    reviews in a pull request.
    let read = Policy::parse(
        "
        PID-3.1  pseudonym    # keep linkage, lose the record number
        PID-5    replace REDACTED
        PID-7    first 4      # the birth year is enough for most tests
        PID-19   clear
        ",
    )?;
    assert_eq!(built.rules, read.rules);

    // Both accept by default: a position no rule names is left alone.
    assert_eq!(built.posture, Posture::Accept);
    assert_eq!(read.posture, Posture::Accept);

    // They differ on one thing, and it is worth knowing about. A payload
    // that is not ER7 at all has no positions in it, so no rule can speak
    // to it. `accept_all` passes one through, because it is a policy that
    // redacts nothing and says so. A policy *file* that mentions no
    // disposition refuses one instead: it was written by somebody who may
    // simply not have considered the case, and refusing loses no value
    // quietly.
    assert_eq!(built.unrecognised, Unrecognised::Pass);
    assert_eq!(read.unrecognised, Unrecognised::Refuse);

    // Either way, say it outright and the two agree.
    let built = built.on_unrecognised(Unrecognised::Refuse);
    assert_eq!(built, read);

    // 3. Start from a built-in and add to it. `--show-policy` on the
    //    command line writes the built-in out as a file to edit.
    let extended = Policy::patient_identifiers()
        .with("NTE-3", Action::Clear)? // free text: nothing positional finds what is in here
        .with("OBX-5", Action::Clear)?;
    assert_eq!(
        extended.rules.len(),
        Policy::patient_identifiers().rules.len() + 2
    );

    // A policy writes itself back out in the file format, so the one that
    // ran can be recorded beside the message it redacted.
    println!("{built}");
    assert_eq!(Policy::parse(&built.to_string())?, built);

    let text = "MSH|^~\\&|LAB\rPID|1||PATID1234||EVERYWOMAN^EVE||19610615|F";
    let mut message = er7::parse(text)?;
    Redactor::new(built).redact(&mut message);
    assert_eq!(message.query("PID-5.1")?.as_deref(), Some("REDACTED"));

    // A malformed policy is rejected at load time, with the line number:
    // a typo here means a value that silently was not redacted.
    let error = Policy::parse("PID-5 obfuscate").unwrap_err();
    println!("{error}");
    assert!(error.to_string().contains("policy line 1"));

    Ok(())
}
