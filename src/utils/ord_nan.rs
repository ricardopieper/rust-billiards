use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
pub struct OrdNan(f64);

impl OrdNan {
    pub fn new(val: f64) -> OrdNan {
        OrdNan(val)
    }
}

impl Eq for OrdNan {}

impl Ord for OrdNan {
    fn cmp(&self, other: &OrdNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}