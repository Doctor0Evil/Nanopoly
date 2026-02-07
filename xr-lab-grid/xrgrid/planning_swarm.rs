use crate::xrlabgrid::nanopoly::nanoswarm::{NanoswarmMember, Nanoswarm};
use crate::store::metrics::ResponseMetric;

impl Nanoswarm {
    pub fn evaluate_planning_session(&self) -> ResponseMetric {
        // Re-use aggregate energy & DW, treat as planning-session envelope
        let base = self.check_policy();
        ResponseMetric::new(
            base.knowledge_factor_k,
            base.demand_d,
            base.dracula_wave_dw,
            "Planning-session aggregate K/D/DW for xr-grid swarm",
        )
    }
}
