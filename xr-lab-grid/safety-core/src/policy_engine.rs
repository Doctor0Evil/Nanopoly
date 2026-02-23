#![forbid(unsafe_code)]

use crate::policy::{CautionCorridors, XRCellEnvelope};
use crate::types::{BioLoadFlag, SafetyState, SwarmMode};

#[derive(Clone, Debug)]
pub struct ActuationProfile {
    pub actuation_scale: f32,   // 0–1 multiplier on mechanical / chemical work
    pub bitrate_scale: f32,     // 0–1 multiplier on comms bitrate
    pub upgrades_locked: bool,  // block high-DW upgrades
}

impl ActuationProfile {
    pub fn normal() -> Self {
        Self {
            actuation_scale: 1.0,
            bitrate_scale: 1.0,
            upgrades_locked: false,
        }
    }

    pub fn cautious() -> Self {
        Self {
            actuation_scale: 0.4,
            bitrate_scale: 0.5,
            upgrades_locked: true,
        }
    }

    pub fn halted() -> Self {
        Self {
            actuation_scale: 0.0,
            bitrate_scale: 0.0,
            upgrades_locked: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PerInstancePolicyOutcome {
    pub effective_mode: SwarmMode,
    pub actuation: ActuationProfile,
    pub notes: &'static str,
}

#[derive(Clone, Debug)]
pub struct NanoswarmPolicyEngine {
    pub corridors: CautionCorridors,
}

impl NanoswarmPolicyEngine {
    pub fn new(corridors: CautionCorridors) -> Self {
        Self { corridors }
    }

    /// Apply per-instance logic given Tsafe's global mode and local SafetyState.
    pub fn evaluate_instance(
        &self,
        tsafe_mode: SwarmMode,
        state: &SafetyState,
    ) -> PerInstancePolicyOutcome {
        match tsafe_mode {
            SwarmMode::Rollback => PerInstancePolicyOutcome {
                effective_mode: SwarmMode::Rollback,
                actuation: ActuationProfile::halted(),
                notes: "tsafe_forced_rollback",
            },
            SwarmMode::Normal | SwarmMode::Caution => {
                match state.bio_flag {
                    BioLoadFlag::Normal => PerInstancePolicyOutcome {
                        effective_mode: SwarmMode::Normal,
                        actuation: ActuationProfile::normal(),
                        notes: "normal_operation",
                    },
                    BioLoadFlag::Violation => PerInstancePolicyOutcome {
                        // Tsafe should have already forced rollback, but double-guard.
                        effective_mode: SwarmMode::Rollback,
                        actuation: ActuationProfile::halted(),
                        notes: "local_violation_halt",
                    },
                    BioLoadFlag::Caution => {
                        // Cautious continuation only if in defined corridor and RoH under hard limit.
                        if self
                            .corridors
                            .is_caution_band(state.k, state.d, state.dw)
                        {
                            PerInstancePolicyOutcome {
                                effective_mode: SwarmMode::Caution,
                                actuation: ActuationProfile::cautious(),
                                notes: "cautious_continuation",
                            }
                        } else {
                            // Outside corridor: still safe but too close to edges, lean conservative.
                            PerInstancePolicyOutcome {
                                effective_mode: SwarmMode::Caution,
                                actuation: ActuationProfile {
                                    actuation_scale: 0.2,
                                    bitrate_scale: 0.3,
                                    upgrades_locked: true,
                                },
                                notes: "caution_outside_soft_corridor",
                            }
                        }
                    }
                }
            }
        }
    }

    /// XR-grid move enforcement: check projected D/Lifeforce against XRCell envelope.
    pub fn check_move(
        &self,
        cell_envelope: &XRCellEnvelope,
        projected_state: &SafetyState,
    ) -> bool {
        cell_envelope.allows_move(projected_state.d, projected_state.lifeforce)
    }
}
