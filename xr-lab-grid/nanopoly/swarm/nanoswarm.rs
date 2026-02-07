use crate::core::{nanopoly_object::NanopolyObject, metrics::ResponseMetric};

#[derive(Clone, Debug)]
pub struct Nanoswarm {
    pub id: String,
    pub members: Vec<NanopolyObject>,
    pub max_energy_d: f32,
    pub max_dw: f32,
}

impl Nanoswarm {
    pub fn check_policy(&self) -> ResponseMetric {
        // Aggregate simple risk model
        let total_energy: f64 = self
            .members
            .iter()
            .map(|m| m.energy.basal_glucose_uW)
            .sum();

        let approximate_d = (total_energy / (1_000_000.0_f64)).min(1.0) as f32;
        let approximate_dw = (approximate_d * 0.5).min(1.0); // conservative mapping

        ResponseMetric::new(
            0.85,
            approximate_d,
            approximate_dw,
            "Nanoswarm aggregate energy and psych-compliance estimate.",
        )
    }
}
