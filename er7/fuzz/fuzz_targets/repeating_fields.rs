//! A repeating field built with `set` survives rendering and reparsing:
//! every value comes back in its own repetition, in order, and none of them
//! turns into the explicit null (R9, R10, R14, R16).
//!
//! The input, split on NUL, is the list of values. They are written through
//! `Subcomponent::set`, so delimiters, terminators, escape characters, and
//! the exact text `""` in a value must all be encoded rather than change the
//! structure or the meaning.
#![forbid(unsafe_code)]
#![no_main]

use er7::message::NULL;
use er7::{Component, Field, Repetition, Segment, Subcomponent};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let values: Vec<&str> = text.split('\u{0}').take(512).collect();
    // A field whose only repetition is empty renders as an empty field,
    // which has no repetitions at all: a different, correct shape.
    if values == [""] {
        return;
    }
    let mut message = er7::parse("MSH|^~\\&|LAB").expect("the header parses");
    let separators = message.separators;
    let mut field = Field::default();
    for value in &values {
        let mut leaf = Subcomponent::default();
        leaf.set(value, &separators);
        field.repetitions.push(Repetition {
            components: vec![Component {
                subcomponents: vec![leaf],
            }],
        });
    }
    message.segments.push(Segment {
        name: "ZRF".into(),
        fields: vec![field],
    });

    let wire = message.to_er7();
    let reparsed = er7::parse(&wire).expect("a rendered message parses again");
    let field = reparsed
        .segment("ZRF")
        .and_then(|segment| segment.field(1))
        .expect("the field is still there");
    assert_eq!(
        field.repetitions.len(),
        values.len(),
        "repetitions were split or merged"
    );
    for (repetition, value) in field.repetitions.iter().zip(&values) {
        let leaves: Vec<&Subcomponent> = repetition
            .components
            .iter()
            .flat_map(|component| &component.subcomponents)
            .collect();
        assert_eq!(leaves.len(), 1, "a value was split: {value:?}");
        assert_eq!(leaves[0].value(&separators), *value);
        assert!(!leaves[0].is_null(), "text became the explicit null");
        if *value == NULL {
            assert!(!repetition.is_null());
        }
    }
});
