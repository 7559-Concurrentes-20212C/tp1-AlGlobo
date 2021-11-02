use std::fmt;

#[derive(Clone)]
pub enum ReservationKind {
    Flight,
    Package,
}

impl fmt::Display for ReservationKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReservationKind::Flight => write!(f, "Flight"),
            ReservationKind::Package => write!(f, "Package"),
        }
    }
}
