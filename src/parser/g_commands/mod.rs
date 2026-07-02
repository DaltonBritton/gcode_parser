pub mod g1;
pub mod g28;
pub mod g29;
pub mod g90;
pub mod g91;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct G92Params {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    e: Option<f64>,
}
