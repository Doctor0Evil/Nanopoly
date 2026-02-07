use crate::store::upgrade_store::{UpgradeModule, UpgradeStore};
use crate::xr_lab_grid::nanopoly::nanopolygon::{
    BiophysicalMetadata, BioAffinityTarget, Nanopolygon, SurfaceCharge, VertexNm, Edge,
};
use crate::xr_lab_grid::nanopoly::nanoswarm::{Nanoswarm, NanoswarmMember};

pub struct XrSession {
    pub swarm: Nanoswarm,
    pub store: UpgradeStore,
}

impl XrSession {
    pub fn new(session_id: &str) -> Self {
        let mut store = UpgradeStore::new();

        store.inventory.push(UpgradeModule {
            id: "mod_neuro_safe_01".to_string(),
            label: "Neuro-safe cortical interface v1".to_string(),
            required_citizen_stake: 10,
            delta_energy_d: 0.05,
            delta_dw: 0.10,
            allowed_targets: vec![BioAffinityTarget::NeuralMembrane],
            max_allowed_charge: SurfaceCharge::Neutral,
        });

        let swarm = Nanoswarm::new(session_id);

        Self { swarm, store }
    }

    pub fn spawn_nanopolygon_member(&mut self) {
        let vertices = vec![
            VertexNm { x_nm: 0.0, y_nm: 0.0, z_nm: 0.0 },
            VertexNm { x_nm: 50.0, y_nm: 0.0, z_nm: 0.0 },
            VertexNm { x_nm: 25.0, y_nm: 43.3, z_nm: 0.0 },
        ];

        let edges = vec![
            Edge { start_index: 0, end_index: 1 },
            Edge { start_index: 1, end_index: 2 },
            Edge { start_index: 2, end_index: 0 },
        ];

        let bio = BiophysicalMetadata {
            target: BioAffinityTarget::NeuralMembrane,
            surface_charge: SurfaceCharge::Neutral,
            hydrophobicity_index: 0.4,
            elastic_modulus_kpa: 5.0,
        };

        let poly = Nanopolygon::new("poly_tri_01", vertices, edges, bio);

        let member = NanoswarmMember {
            poly,
            basal_glucose_uW: 100.0,
        };

        self.swarm.add_member(member);
    }

    pub fn evaluate_swarm(&self) -> crate::store::metrics::ResponseMetric {
        self.swarm.check_policy()
    }

    pub fn evaluate_first_member_upgrade(&self) -> Option<crate::store::upgrade_store::UpgradeDecision> {
        let member = self.swarm.members.first()?;
        let module = self.store.inventory.first()?;
        Some(self.store.evaluate_upgrade(&member.poly, module))
    }
}
