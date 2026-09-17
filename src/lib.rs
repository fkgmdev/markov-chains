use serde::{Deserialize, Serialize};

use crate::logic::is_vowel;
pub mod analyze;
pub mod args;
pub mod logic;
#[cfg(test)]
pub mod tests;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub vvvp: f64,
    pub vvcp: f64,
    pub vcvp: f64,
    pub vccp: f64,
    pub cvvp: f64,
    pub cvcp: f64,
    pub ccvp: f64,
    pub cccp: f64,
    pub cvrp: f64,
}

impl Profile {
    pub fn to_features(&self) -> [f64; 9] {
        [
            self.vvvp, self.vvcp, self.vcvp, self.vccp, self.cvvp, self.cvcp, self.ccvp, self.cccp,
            self.cvrp,
        ]
    }
    pub fn merge_with(&mut self, other: &Profile) {
        self.vvvp = (self.vvvp + other.vvvp) / 2.0;
        self.vvcp = (self.vvcp + other.vvcp) / 2.0;
        self.vcvp = (self.vcvp + other.vcvp) / 2.0;
        self.vccp = (self.vccp + other.vccp) / 2.0;
        self.cvvp = (self.cvvp + other.cvvp) / 2.0;
        self.cvcp = (self.cvcp + other.cvcp) / 2.0;
        self.ccvp = (self.ccvp + other.ccvp) / 2.0;
        self.cccp = (self.cccp + other.cccp) / 2.0;
        self.cvrp = (self.cvrp + other.cvrp) / 2.0;
    }
}
