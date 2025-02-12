use nalgebra::DVector;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SimulatePriceVerifyPositionInput {
    pub positions: Vec<f64>,
    pub pt: DVector<f64>,
    pub pt_1: DVector<f64>,
    pub gradient_tolerance: f64,
    pub de_seasonalised_detrended_log_base_fee: DVector<f64>,
    pub n_periods: usize,
    pub num_paths: usize,
}
