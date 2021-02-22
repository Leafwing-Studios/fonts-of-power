use derive_more::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Attribute {
    Any,
    Prowess,
    Agility,
    Expertise,
    Focus,
    Presence,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct AttributeVal(i8);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Attributes {
    prowess: AttributeVal,
    agility: AttributeVal,
    expertise: AttributeVal,
    focus: AttributeVal,
    presence: AttributeVal,
}
