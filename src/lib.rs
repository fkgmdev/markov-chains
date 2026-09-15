use crate::logic::is_vowel;
use itertools::Itertools;
use std::fs;
pub mod analyze;
pub mod logic;

struct Profile {
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
