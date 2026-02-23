#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum BioLoadFlag {
    Normal,
    Caution,   // local "!" soft limit, cautious continuation
    Violation, // hard breach
}

#[derive(Clone, Debug, PartialEq)]
pub enum SwarmMode {
    Normal,
    Caution,
    Rollback,
}

/// Rights-of-Humanity scalar, 0.0 – 1.0 (higher = more rights pressure / risk).
/// Hard constraint: roh <= 0.3 for any action to be allowed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RightsOfHumanity(pub f32);

impl RightsOfHumanity {
    pub fn clamped(v: f32) -> Self {
        Self(v.clamp(0.0, 1.0))
    }

    pub fn is_within_limit(&self, limit: f32) -> bool {
        self.0 <= limit
    }
}

/// Composite LifeforceIndex 0.0–1.0 (higher = healthier, more sustainable).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LifeforceIndex(pub f32);

impl LifeforceIndex {
    pub fn clamped(v: f32) -> Self {
        Self(v.clamp(0.0, 1.0))
    }

    pub fn is_above(&self, min: f32) -> bool {
        self.0 >= min
    }
}

/// Per-instance safety snapshot for a nanoswarm member.
#[derive(Clone, Debug)]
pub struct SafetyState {
    /// Knowledge factor K (epistemic confidence / clarity).
    pub k: f32,
    /// Normalized host energy demand D (0–1 corridor).
    pub d: f32,
    /// Psych-risk / DraculaWave DW (0–1 leverage).
    pub dw: f32,
    /// Composite biophysical vitality.
    pub lifeforce: LifeforceIndex,
    /// Rights-of-Humanity pressure.
    pub roh: RightsOfHumanity,
    /// Local bioload classifier.
    pub bio_flag: BioLoadFlag,
    /// Controller posture at this instance.
    pub swarm_mode: SwarmMode,
}

impl SafetyState {
    pub fn new(
        k: f32,
        d: f32,
        dw: f32,
        lifeforce: f32,
        roh: f32,
        bio_flag: BioLoadFlag,
        swarm_mode: SwarmMode,
    ) -> Self {
        Self {
            k: k.clamp(0.0, 1.0),
            d: d.clamp(0.0, 1.0),
            dw: dw.clamp(0.0, 1.0),
            lifeforce: LifeforceIndex::clamped(lifeforce),
            roh: RightsOfHumanity::clamped(roh),
            bio_flag,
            swarm_mode,
        }
    }
}

/// Aggregated swarm view used by the Tsafe Cortex Gate.
#[derive(Clone, Debug)]
pub struct AggregatedSafetyState {
    pub avg_k: f32,
    pub avg_d: f32,
    pub avg_dw: f32,
    pub min_lifeforce: LifeforceIndex,
    pub max_roh: RightsOfHumanity,
    pub any_violation: bool,
}

impl AggregatedSafetyState {
    pub fn from_instances(instances: &[SafetyState]) -> Self {
        let n = instances.len().max(1) as f32;
        let mut sum_k = 0.0;
        let mut sum_d = 0.0;
        let mut sum_dw = 0.0;
        let mut min_lifeforce = 1.0_f32;
        let mut max_roh = 0.0_f32;
        let mut any_violation = false;

        for s in instances {
            sum_k += s.k;
            sum_d += s.d;
            sum_dw += s.dw;
            min_lifeforce = min_lifeforce.min(s.lifeforce.0);
            max_roh = max_roh.max(s.roh.0);
            if matches!(s.bio_flag, BioLoadFlag::Violation) {
                any_violation = true;
            }
        }

        Self {
            avg_k: (sum_k / n).clamp(0.0, 1.0),
            avg_d: (sum_d / n).clamp(0.0, 1.0),
            avg_dw: (sum_dw / n).clamp(0.0, 1.0),
            min_lifeforce: LifeforceIndex::clamped(min_lifeforce),
            max_roh: RightsOfHumanity::clamped(max_roh),
            any_violation,
        }
    }
}
