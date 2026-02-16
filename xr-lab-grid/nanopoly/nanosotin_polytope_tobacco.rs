#[derive(Clone, Debug)]
pub enum DetoxPhase {
    Preparation,      // -2 to 0 weeks
    AcuteWithdrawal,  // 0–7 days
    EarlyRecovery,    // 1–4 weeks
    NeuroReversal,    // 1–3 months
    StableRecovery,   // 3–12+ months
}

#[derive(Clone, Debug)]
pub enum RohBand {
    Calm,      // RoH ≤ 0.20
    Warning,   // 0.20 < RoH ≤ 0.30
    HardStop,  // RoH > 0.30 (should not be entered in this profile)
}

#[derive(Clone, Debug)]
pub enum AdvisoryMode {
    FullExplore,    // rich explanations, options allowed
    WarnMinimal,    // short, simple, rest-biased
    ExplainOnly,    // no new goals; suggest rest and stabilization
}

#[derive(Clone, Debug)]
pub struct DetoxPhaseProfile {
    pub phase: DetoxPhase,
    pub roh_band: RohBand,
    pub k_min: f32,
    pub k_max: f32,
    pub d_min: f32,
    pub d_max: f32,
    pub dw_min: f32,
    pub dw_max: f32,
    pub notes: String,
}

#[derive(Clone, Debug)]
pub struct NanosotinPolytopeTobacco {
    pub id: String,
    pub profiles: Vec<DetoxPhaseProfile>,
    /// Global non-commercial flag: this polytope may not be used
    /// for profit, paywalls, or coercive monetization.
    pub non_commercial_only: bool,
}

impl NanosotinPolytopeTobacco {
    pub fn new() -> Self {
        let profiles = vec![
            DetoxPhaseProfile {
                phase: DetoxPhase::Preparation,
                roh_band: RohBand::Calm,
                k_min: 0.92,
                k_max: 0.97,
                d_min: 0.20,
                d_max: 0.30,
                dw_min: 0.08,
                dw_max: 0.12,
                notes: "Planning, education, trigger mapping. Eco-wealth framing allowed.".to_string(),
            },
            DetoxPhaseProfile {
                phase: DetoxPhase::AcuteWithdrawal,
                roh_band: RohBand::Warning,
                k_min: 0.88,
                k_max: 0.95,
                d_min: 0.35,
                d_max: 0.55,
                dw_min: 0.14,
                dw_max: 0.20,
                notes: "High metabolic and psychological load. Short, rest-first messages only.".to_string(),
            },
            DetoxPhaseProfile {
                phase: DetoxPhase::EarlyRecovery,
                roh_band: RohBand::Warning,
                k_min: 0.90,
                k_max: 0.96,
                d_min: 0.25,
                d_max: 0.40,
                dw_min: 0.12,
                dw_max: 0.18,
                notes: "Rebuilding routines, gentle identity work, eco-benefits reinforcement.".to_string(),
            },
            DetoxPhaseProfile {
                phase: DetoxPhase::NeuroReversal,
                roh_band: RohBand::Calm,
                k_min: 0.92,
                k_max: 0.97,
                d_min: 0.18,
                d_max: 0.30,
                dw_min: 0.10,
                dw_max: 0.16,
                notes: "Neuroadaptation reversal; richer education allowed, still low coercion.".to_string(),
            },
            DetoxPhaseProfile {
                phase: DetoxPhase::StableRecovery,
                roh_band: RohBand::Calm,
                k_min: 0.92,
                k_max: 0.98,
                d_min: 0.10,
                d_max: 0.22,
                dw_min: 0.08,
                dw_max: 0.14,
                notes: "Long-term relapse prevention, eco-wealth and community focus.".to_string(),
            },
        ];

        Self {
            id: "NanosotinPolytope_TobaccoDetox_v1".to_string(),
            profiles,
            non_commercial_only: true,
        }
    }

    pub fn get_profile(&self, phase: &DetoxPhase) -> Option<&DetoxPhaseProfile> {
        self.profiles.iter().find(|p| &p.phase == phase)
    }

    /// Recommend advisory mode given phase, current D, DW, and fatigue index FI.
    /// FI is 0–1, where higher = more fatigued.
    pub fn recommend_mode(
        &self,
        phase: &DetoxPhase,
        d: f32,
        dw: f32,
        fatigue_index: f32,
    ) -> AdvisoryMode {
        let fi = fatigue_index.clamp(0.0, 1.0);
        let d_clamped = d.clamp(0.0, 1.0);
        let dw_clamped = dw.clamp(0.0, 1.0);

        // Hard fatigue safeguard: if FI or D are high, force ExplainOnly.
        if fi > 0.70 || d_clamped > 0.60 {
            return AdvisoryMode::ExplainOnly;
        }

        // Psych-risk safeguard: if DW is high, avoid high-leverage guidance.
        if dw_clamped > 0.22 {
            return AdvisoryMode::ExplainOnly;
        }

        match phase {
            DetoxPhase::Preparation => {
                if fi < 0.40 && d_clamped < 0.40 && dw_clamped < 0.18 {
                    AdvisoryMode::FullExplore
                } else {
                    AdvisoryMode::WarnMinimal
                }
            }
            DetoxPhase::AcuteWithdrawal => {
                if fi < 0.50 && d_clamped < 0.45 && dw_clamped < 0.20 {
                    AdvisoryMode::WarnMinimal
                } else {
                    AdvisoryMode::ExplainOnly
                }
            }
            DetoxPhase::EarlyRecovery => {
                if fi < 0.45 && d_clamped < 0.40 && dw_clamped < 0.20 {
                    AdvisoryMode::FullExplore
                } else {
                    AdvisoryMode::WarnMinimal
                }
            }
            DetoxPhase::NeuroReversal | DetoxPhase::StableRecovery => {
                if fi < 0.40 && d_clamped < 0.35 && dw_clamped < 0.18 {
                    AdvisoryMode::FullExplore
                } else {
                    AdvisoryMode::WarnMinimal
                }
            }
        }
    }

    /// Explicit guardrail: this polytope MUST NOT be bound to for-profit flows.
    pub fn enforce_non_commercial(&self, is_monetized: bool) -> bool {
        if self.non_commercial_only && is_monetized {
            // Deny binding if any monetization is detected.
            return false;
        }
        true
    }
}
