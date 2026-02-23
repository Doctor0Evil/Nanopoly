#![forbid(unsafe_code)]

use safety_core::ml_bridge::{SafetyModel, SensorFeatures};
use safety_core::policy::{CautionCorridors, HardLimits, XRCellEnvelope};
use safety_core::policy_engine::NanoswarmPolicyEngine;
use safety_core::tsafe_cortex_gate::TsafeCortexGate;
use safety_core::types::{AggregatedSafetyState, SafetyState, SwarmMode};

pub struct XRNavController<M: SafetyModel> {
    tsafe: TsafeCortexGate,
    policy: NanoswarmPolicyEngine,
    model: M,
}

impl<M: SafetyModel> XRNavController<M> {
    pub fn new(model: M, limits: HardLimits, corridors: CautionCorridors) -> Self {
        Self {
            tsafe: TsafeCortexGate::new(limits),
            policy: NanoswarmPolicyEngine::new(corridors),
            model,
        }
    }

    /// Evaluate a proposed move for a small sub-swarm through one XR cell.
    pub fn evaluate_move(
        &self,
        features_per_member: &[SensorFeatures],
        cell_envelope: &XRCellEnvelope,
    ) -> (SwarmMode, bool) {
        // 1) Predict SafetyState per member from sensor features.
        let mut states: Vec<SafetyState> = Vec::with_capacity(features_per_member.len());
        for f in features_per_member {
            let s = self.model.predict_safety(f);
            states.push(s);
        }

        // 2) Aggregate for Tsafe.
        let agg = AggregatedSafetyState::from_instances(&states);

        // 3) Tsafe decides global mode.
        let tsafe_decision = self.tsafe.evaluate(&agg);
        let enforced_mode = tsafe_decision.enforced_mode;

        // 4) Policy engine checks whether the move is allowed in this cell.
        //    We use the worst projected D/Lifeforce for this sub-swarm.
        let mut max_d = 0.0_f32;
        let mut min_lf = 1.0_f32;
        for s in &states {
            max_d = max_d.max(s.d);
            min_lf = min_lf.min(s.lifeforce.0);
        }
        let projected_state = SafetyState::new(
            agg.avg_k,
            max_d,
            agg.avg_dw,
            min_lf,
            agg.max_roh.0,
            // choose the strictest flag we observed
            if agg.any_violation {
                crate::types::BioLoadFlag::Violation
            } else {
                crate::types::BioLoadFlag::Caution
            },
            enforced_mode,
        );

        let move_allowed = match enforced_mode {
            SwarmMode::Rollback => false,
            SwarmMode::Normal | SwarmMode::Caution => {
                self.policy.check_move(cell_envelope, &projected_state)
            }
        };

        (enforced_mode, move_allowed)
    }
}
