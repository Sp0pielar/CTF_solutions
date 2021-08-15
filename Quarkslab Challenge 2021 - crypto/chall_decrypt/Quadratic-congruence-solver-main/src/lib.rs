mod euclid;
mod prime;
mod residue;

pub mod solver;

pub const MAX_COEF_VAL: i64 = i64::MAX - 1;
pub const MIN_COEF_VAL: i64 = i64::MIN + 2;

pub struct Coefs {
    pub a: i64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub n: i64,
}

pub enum Equation {
    Linear(Coefs),
    Quad(Coefs),
}
