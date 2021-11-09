use std::fmt;

#[derive(Copy, Clone)]
pub enum WebserviceKind {
    Airline,
    Hotel,
    Merge,
}

impl fmt::Display for WebserviceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WebserviceKind::Airline => write!(f, "Airline"),
            WebserviceKind::Hotel => write!(f, "Hotel"),
            WebserviceKind::Merge => write!(f, "Merge"),
        }
    }
}
