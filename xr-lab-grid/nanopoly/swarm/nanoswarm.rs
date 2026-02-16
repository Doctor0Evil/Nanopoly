use crate::core::{nanopoly_object::NanopolyObject, metrics::ResponseMetric};
use crate::core::species::{HostType, SpeciesId, HostBudgetProfile};
use crate::core::governance::GovernanceLayer;
use crate::store::metrics::ResponseMetric;

#[derive(Clone, Debug)]
pub struct Nanoswarm {
    pub id: String,
    pub members: Vec<NanopolyObject>,
    pub max_energy_d: f32,
    pub max_dw: f32,
    pub host_budget: HostBudgetProfile,
    pub governance: GovernanceLayer,
}

impl Nanoswarm {
    pub fn check_policy(&self, metric: &ResponseMetric) -> bool {
        // Species-aware ceilings
        let d_ok = metric.demand_d <= self.host_budget.d_max;
        let dw_ok = metric.draculawave_dw <= self.host_budget.dw_max;

        // Non-commercial, no-entertainment guard for non-humans
        let econ_ok = match self.governance.host_type {
            HostType::Human => true,
            _ => self.governance.non_commercial && self.governance.no_entertainment,
        };

        d_ok && dw_ok && econ_ok
    }
}


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
