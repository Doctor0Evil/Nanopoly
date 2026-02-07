use crate::store::metrics::ResponseMetric;
use crate::xr_lab_grid::nanopoly::nanopolygon::{
    Nanopolygon, SurfaceCharge, BioAffinityTarget,
};

#[derive(Clone, Debug)]
pub struct UpgradeModule {
    pub id: String,
    pub label: String,
    pub required_citizen_stake: u64,
    pub delta_energy_d: f32,
    pub delta_dw: f32,
    pub allowed_targets: Vec<BioAffinityTarget>,
    pub max_allowed_charge: SurfaceCharge,
}

#[derive(Clone, Debug)]
pub struct UpgradeDecision {
    pub poly_id: String,
    pub module_id: String,
    pub metric: ResponseMetric,
    pub allowed: bool,
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
        poly: &Nanopolygon,
        module: &UpgradeModule,
    ) -> UpgradeDecision {
        let target_ok = module
            .allowed_targets
            .iter()
            .any(|t| *t as i32 == poly.bio.target as i32);

        let charge_ok = match (&poly.bio.surface_charge, &module.max_allowed_charge) {
            (SurfaceCharge::Positive, SurfaceCharge::Neutral) => false,
            (SurfaceCharge::Positive, SurfaceCharge::Negative) => false,
            _ => true,
        };

        let allowed = target_ok && charge_ok;

        let d = module.delta_energy_d;
        let dw = module.delta_dw;
        let k = if allowed { 0.92 } else { 0.78 };

        let notes = if allowed {
            "Upgrade within nanopolygon biophysical and cortical constraints."
        } else {
            "Upgrade violates nanopolygon biophysical or cortical constraints."
        };

        let metric = ResponseMetric::new(k, d, dw, notes);

        UpgradeDecision {
            poly_id: poly.id.clone(),
            module_id: module.id.clone(),
            metric,
            allowed,
        }
    }
}
