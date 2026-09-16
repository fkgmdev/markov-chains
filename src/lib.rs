use crate::logic::is_vowel;
pub mod analyze;
pub mod logic;
#[cfg(test)]
pub mod tests;

pub struct Profile {
    name: String,
    vvvp: f64,
    vvcp: f64,
    vcvp: f64,
    vccp: f64,
    cvvp: f64,
    cvcp: f64,
    ccvp: f64,
    cccp: f64,
    cvrp: f64,
}

impl Profile {
    pub fn to_features(&self) -> [f64; 9] {
        [
            self.vvvp, self.vvcp, self.vcvp, self.vccp, self.cvvp, self.cvcp, self.ccvp, self.cccp,
            self.cvrp,
        ]
    }
}
