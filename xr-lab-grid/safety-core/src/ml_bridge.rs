#![forbid(unsafe_code)]

use crate::types::{BioLoadFlag, LifeforceIndex, RightsOfHumanity, SafetyState, SwarmMode};

/// Minimal feature vector aligned with on-device model.
#[derive(Clone, Debug)]
pub struct SensorFeatures {
    pub d: f32,   // Host Energy Demand
    pub tdi: f32, // ThermalDistanceIndex
    pub mbi: f32, // MolecularBalanceIndex
    pub dw: f32,  // local psych-risk
    pub lifeforce: f32,
}

/// Interface that your embedded ML runtime should implement.
pub trait SafetyModel {
    fn predict_safety(&self, features: &SensorFeatures) -> SafetyState;
}

/// Example deterministic wrapper around a model output.
/// Replace internals with real model bindings (e.g., via FFI or no_std runtime).
pub struct SimpleSafetyModel;

impl SimpleSafetyModel {
    fn classify_bio_flag(d: f32, lifeforce: f32, tdi: f32, mbi: f32) -> BioLoadFlag {
        // Example: high thermal/molecular stress or low lifeforce -> Caution/Violation.
        if lifeforce < 0.4 || d > 0.8 || tdi > 0.8 || mbi < 0.2 {
            BioLoadFlag::Violation
        } else if lifeforce < 0.6 || d > 0.5 || tdi > 0.6 || mbi < 0.4 {
            BioLoadFlag::Caution
        } else {
            BioLoadFlag::Normal
        }
    }

    fn infer_k_uncertainty(d: f32, tdi: f32, mbi: f32) -> f32 {
        // Higher stress => lower epistemic confidence.
        let stress = 0.5 * d + 0.25 * tdi + 0.25 * (1.0 - mbi);
        (1.0 - stress).clamp(0.0, 1.0)
    }
}

impl SafetyModel for SimpleSafetyModel {
    fn predict_safety(&self, f: &SensorFeatures) -> SafetyState {
        let bio_flag = Self::classify_bio_flag(f.d, f.lifeforce, f.tdi, f.mbi);
        let k = Self::infer_k_uncertainty(f.d, f.tdi, f.mbi);
        let roh = 0.2_f32; // placeholder: wire to consent & governance telemetry.

        SafetyState::new(
            k,
            f.d,
            f.dw,
            f.lifeforce,
            roh,
            bio_flag,
            SwarmMode::Normal,
        )
    }
}
