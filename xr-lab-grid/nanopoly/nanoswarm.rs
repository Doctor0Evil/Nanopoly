use crate::store::metrics::ResponseMetric;
use super::nanopolygon::Nanopolygon;

#[derive(Clone, Debug)]
pub struct NanoswarmMember {
    pub poly: Nanopolygon,
    pub basal_glucose_uW: f64,
}

#[derive(Clone, Debug)]
pub struct Nanoswarm {
    pub id: String,
    pub members: Vec<NanoswarmMember>,
    pub max_energy_d: f32,
    pub max_dw: f32,
}

impl Nanoswarm {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            members: Vec::new(),
            max_energy_d: 1.0,
            max_dw: 1.0,
        }
    }

    pub fn add_member(&mut self, member: NanoswarmMember) {
        self.members.push(member);
    }

    pub fn total_energy_uW(&self) -> f64 {
        self.members
            .iter()
            .map(|m| m.basal_glucose_uW)
            .sum()
    }

    pub fn check_policy(&self) -> ResponseMetric {
        let total_energy = self.total_energy_uW();
        let approximate_d = (total_energy / 1_000_000.0_f64).min(1.0) as f32;
        let approximate_dw = (approximate_d * 0.5).min(1.0);

        ResponseMetric::new(
            0.85,
            approximate_d,
            approximate_dw,
            "Nanoswarm aggregate energy and psych-compliance estimate.",
        )
    }
}
