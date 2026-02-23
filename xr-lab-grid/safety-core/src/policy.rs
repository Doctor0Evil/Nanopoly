#![forbid(unsafe_code)]

use crate::types::{BioLoadFlag, LifeforceIndex, RightsOfHumanity, SwarmMode};

/// Static hard limits (Tsafe-level, non-negotiable).
#[derive(Clone, Debug)]
pub struct HardLimits {
    /// Max legal average D for this host profile.
    pub max_d: f32,
    /// Min acceptable LifeforceIndex.
    pub min_lifeforce: f32,
    /// Max legal RoH (e.g., 0.3).
    pub max_roh: f32,
    /// Max allowed DW before rollback enforced.
    pub max_dw: f32,
}

impl HardLimits {
    pub fn clinical_default() -> Self {
        Self {
            max_d: 0.35,          // conservative for intensive modes
            min_lifeforce: 0.60,  // require good resilience
            max_roh: 0.30,        // Rights-of-Humanity ceiling
            max_dw: 0.25,         // cap psych-risk
        }
    }

    pub fn everyday_bci() -> Self {
        Self {
            max_d: 0.25,
            min_lifeforce: 0.50,
            max_roh: 0.30,
            max_dw: 0.20,
        }
    }
}

/// Soft corridors for cautious continuation (PolicyEngine-level).
#[derive(Clone, Debug)]
pub struct CautionCorridors {
    pub caution_d_low: f32,
    pub caution_d_high: f32,
    pub caution_dw_low: f32,
    pub caution_dw_high: f32,
    pub caution_k_min: f32,
}

impl CautionCorridors {
    pub fn default() -> Self {
        Self {
            // host is working but not overloaded
            caution_d_low: 0.20,
            caution_d_high: 0.40,
            // psych-risk corridor
            caution_dw_low: 0.10,
            caution_dw_high: 0.30,
            // require decent epistemic confidence
            caution_k_min: 0.70,
        }
    }

    pub fn is_caution_band(&self, k: f32, d: f32, dw: f32) -> bool {
        k >= self.caution_k_min
            && d >= self.caution_d_low
            && d <= self.caution_d_high
            && dw >= self.caution_dw_low
            && dw <= self.caution_dw_high
    }
}

/// Per-instance navigation envelope inside the XR grid.
#[derive(Clone, Debug)]
pub struct XRCellEnvelope {
    pub host_budget_d_remaining: f32, // 0–1 normalized local energy capacity
    pub lifeforce_floor: f32,
}

impl XRCellEnvelope {
    pub fn allows_move(&self, projected_d: f32, projected_lifeforce: LifeforceIndex) -> bool {
        projected_d <= self.host_budget_d_remaining && projected_lifeforce.is_above(self.lifeforce_floor)
    }
}
