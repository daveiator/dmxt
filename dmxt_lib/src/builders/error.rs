#[derive(Debug)]
pub enum BuildError {
    MissingField(&'static str),
    EmptyField(&'static str),
}