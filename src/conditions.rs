pub trait Condition: Clone + std::fmt::Debug + std::cmp::Eq + std::cmp::PartialEq {}
pub trait Affliction: Condition {}
pub trait Ailment: Condition {}
pub trait Status: Condition {}
