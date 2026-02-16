#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Species {
    Human,
    Canine,
    Feline,
    Other(String),
}
