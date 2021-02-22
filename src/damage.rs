#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Element {
    Air,
    Earth,
    Fire,
    Water,
    Eldritch,
    Arcane,
    Radiant,
    Umbral,
    Primal,
    Decay,
    Electric,
    Corrosive,
}

pub enum DamageType {
    Physical,
    Elemental(Element),
}
