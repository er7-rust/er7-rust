//! Private borrowing serializers: the wire shape of spec §2, written straight
//! from the `er7` tree without cloning it into wrapper types first.
//!
//! Every public `Serialize` impl in this crate delegates here. That keeps
//! one owning wrapper per tree level as the public API (spec §6.4) while
//! serialization itself copies nothing. Before this, each level cloned its
//! children to wrap them, which doubled peak memory when serializing a
//! large message (see `BENCHMARKS.md`).

use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};

use crate::Separators;

/// A borrowed `er7` node, serialized exactly as its owning wrapper would be.
pub(crate) struct Ref<'a, T>(pub(crate) &'a T);

/// A borrowed list of nodes, serialized as one sequence with a known length
/// (which a positional format such as bincode requires).
struct Seq<'a, T>(&'a [T]);

impl<'a, T> Serialize for Seq<'a, T>
where
    Ref<'a, T>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for item in self.0 {
            seq.serialize_element(&Ref(item))?;
        }
        seq.end()
    }
}

impl Serialize for Ref<'_, er7::Subcomponent> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.raw)
    }
}

impl Serialize for Ref<'_, er7::Component> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Seq(&self.0.subcomponents).serialize(serializer)
    }
}

impl Serialize for Ref<'_, er7::Repetition> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Seq(&self.0.components).serialize(serializer)
    }
}

impl Serialize for Ref<'_, er7::Field> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Seq(&self.0.repetitions).serialize(serializer)
    }
}

impl Serialize for Ref<'_, er7::Segment> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Segment", 2)?;
        state.serialize_field("name", &self.0.name)?;
        state.serialize_field("fields", &Seq(&self.0.fields))?;
        state.end()
    }
}

impl Serialize for Ref<'_, er7::Message> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Message", 2)?;
        state.serialize_field("separators", &Separators(self.0.separators))?;
        state.serialize_field("segments", &Seq(&self.0.segments))?;
        state.end()
    }
}
