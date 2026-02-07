use crate::xrlabgrid::nanopoly::nanopolygon::{Nanopolygon, BiophysicalMetadata};
use crate::store::metrics::ResponseMetric;
use crate::store::upgradestore::UpgradeModule;

#[derive(Clone, Debug)]
pub struct XrGridCell {
    pub id: String,
    pub poly: Nanopolygon,
    pub shard_path: String,      // maps to QPU.Datashard destination-path
    pub service_endpoint: String // Rust microservice URL for this cell
}

impl XrGridCell {
    pub fn evaluate_upgrade(
        &self,
        module: &UpgradeModule,
    ) -> ResponseMetric {
        // Delegate to UpgradeStore semantics: K/D/DW based on biophysics + module deltas
        let allowed_targets = &module.allowed_targets;
        let charge_ok = module.max_allowed_charge >= self.poly.bio.surfacecharge;
        let target_ok = allowed_targets.iter().any(|t| *t == self.poly.bio.target);

        let (k, d, dw, notes) = if charge_ok && target_ok {
            (0.90, module.delta_energy_d, module.delta_dw,
             "XR-grid cell upgrade within nanopolygon constraints")
        } else {
            (0.70, module.delta_energy_d, 0.30_f32.max(module.delta_dw),
             "XR-grid cell upgrade violates nanopolygon constraints")
        };

        ResponseMetric::new(k, d, dw, notes)
    }
}
