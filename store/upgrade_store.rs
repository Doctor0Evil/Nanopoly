use crate::core::{nanopoly_object::NanopolyObject, metrics::ResponseMetric};

#[derive(Clone, Debug)]
pub struct UpgradeModule {
    pub id: String,
    pub label: String,
    pub required_citizen_stake: u64,
    pub delta_energy_d: f32,
    pub delta_dw: f32,
}

#[derive(Clone, Debug)]
pub struct UpgradeDecision {
    pub object_id: String,
    pub module_id: String,
    pub metric: ResponseMetric,
}

pub struct UpgradeStore {
    pub inventory: Vec<UpgradeModule>,
}

impl UpgradeStore {
    pub fn new() -> Self {
        Self { inventory: Vec::new() }
    }

    pub fn list_modules(&self) -> &Vec<UpgradeModule> {
        &self.inventory
    }

    pub fn evaluate_upgrade(
        &self,
        obj: &NanopolyObject,
        module: &UpgradeModule,
    ) -> UpgradeDecision {
        let k = 0.9; // high knowledge clarity for well-specified modules
        let d = (obj.energy.basal_glucose_uW + module.delta_energy_d as f64) as f32;
        let dw = module.delta_dw;

        let metric = ResponseMetric::new(
            k,
            d,
            dw,
            "Pre-application evaluation of upgrade energy and psych risk.",
        );

        UpgradeDecision {
            object_id: obj.id.clone(),
            module_id: module.id.clone(),
            metric,
        }
    }
}
