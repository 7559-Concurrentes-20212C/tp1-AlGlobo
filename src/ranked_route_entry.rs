use std::fmt;

pub struct RankedRouteEntry {
    pub rank: usize,
    pub route: String,
    pub count: usize,
}

impl fmt::Display for RankedRouteEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}. {} with {} requests",
            self.rank, self.route, self.count
        )
    }
}
