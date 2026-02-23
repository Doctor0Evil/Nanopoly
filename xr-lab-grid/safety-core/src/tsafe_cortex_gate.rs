#![forbid(unsafe_code)]

use crate::policy::HardLimits;
use crate::types::{AggregatedSafetyState, SwarmMode};

#[derive(Clone, Debug)]
pub struct TsafeDecision {
    pub enforced_mode: SwarmMode,
    pub reason: &'static str,
}

#[derive(Clone, Debug)]
pub struct TsafeCortexGate {
    pub limits: HardLimits,
}

impl TsafeCortexGate {
    pub fn new(limits: HardLimits) -> Self {
        Self { limits }
    }

    /// Evaluate aggregated swarm state against non-negotiable constraints.
    pub fn evaluate(&self, agg: &AggregatedSafetyState) -> TsafeDecision {
        // Rule 1: any Violation -> global Rollback.
        if agg.any_violation {
            return TsafeDecision {
                enforced_mode: SwarmMode::Rollback,
                reason: "member_violation_flag",
            };
        }

        // Rule 2: RoH ceiling.
        if !agg.max_roh.is_within_limit(self.limits.max_roh) {
            return TsafeDecision {
                enforced_mode: SwarmMode::Rollback,
                reason: "roh_exceeds_limit",
            };
        }

        // Rule 3: energy budget.
        if agg.avg_d > self.limits.max_d {
            return TsafeDecision {
                enforced_mode: SwarmMode::Rollback,
                reason: "host_energy_budget_exceeded",
            };
        }

        // Rule 4: psych-risk.
        if agg.avg_dw > self.limits.max_dw {
            return TsafeDecision {
                enforced_mode: SwarmMode::Rollback,
                reason: "psych_risk_exceeded",
            };
        }

        // Rule 5: lifeforce floor.
        if !agg.min_lifeforce.is_above(self.limits.min_lifeforce) {
            return TsafeDecision {
                enforced_mode: SwarmMode::Rollback,
                reason: "lifeforce_below_floor",
            };
        }

        // If we pass all hard gates, Tsafe defers nuance to PolicyEngine.
        TsafeDecision {
            enforced_mode: SwarmMode::Normal,
            reason: "within_hard_limits",
        }
    }
}
