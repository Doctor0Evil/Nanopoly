// File: xr-lab-grid/nanopoly/nanosotin_polytope_tobacco.rs

#[derive(Clone, Debug)]
pub enum DetoxPhase {
    Preparation,          // -2 to 0 weeks
    AcuteWithdrawal,      // 0–7 days
    EarlyRecovery,        // 1–4 weeks
    NeuroadaptationRev,   // 1–3 months
    StableRecovery,       // 3–12+ months
}

#[derive(Clone, Debug)]
pub enum RohBand {
    Calm,
    Warning,
}

#[derive(Clone, Debug)]
pub enum UxMode {
    FullExplore,     // normal explanations, options
    WarnMinimal,     // short, rest‑first, low branching
    HardStopExplain, // explain‑only, no planning
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
    pub max_dw_soft: f32,
    pub max_dw_hard: f32,
    pub allowed_modes: Vec<UxMode>,
}

impl DetoxPhaseProfile {
    pub fn recommend_mode(&self, k: f32, d: f32, dw: f32) -> UxMode {
        // Clamp inputs for safety
        let k = k.clamp(0.0, 1.0);
        let d = d.clamp(0.0, 1.0);
        let dw = dw.clamp(0.0, 1.0);

        // Hard RoH guard: if DW or D exceed hard ceilings, force HardStopExplain
        if dw > self.max_dw_hard || d > self.d_max {
            return UxMode::HardStopExplain;
        }

        // Soft guard: if DW is above soft limit, stay in WarnMinimal
        if dw > self.max_dw_soft {
            if self.allowed_modes.contains(&UxMode::WarnMinimal) {
                return UxMode::WarnMinimal;
            }
            return UxMode::HardStopExplain;
        }

        // Within band: pick richest safe mode for this phase
        if self.allowed_modes.contains(&UxMode::FullExplore) {
            UxMode::FullExplore
        } else if self.allowed_modes.contains(&UxMode::WarnMinimal) {
            UxMode::WarnMinimal
        } else {
            UxMode::HardStopExplain
        }
    }
}

#[derive(Clone, Debug)]
pub struct NanosotinPolytopeTobaccoDetoxV1 {
    pub id: String,
    pub profiles: Vec<DetoxPhaseProfile>,
}

impl NanosotinPolytopeTobaccoDetoxV1 {
    pub fn new(id: &str) -> Self {
        use DetoxPhase::*;
        use RohBand::*;
        use UxMode::*;

        let profiles = vec![
            DetoxPhaseProfile {
                phase: Preparation,
                roh_band: Calm,
                k_min: 0.92,
                k_max: 0.96,
                d_min: 0.20,
                d_max: 0.30,
                dw_min: 0.08,
                dw_max: 0.12,
                max_dw_soft: 0.15,
                max_dw_hard: 0.20,
                allowed_modes: vec![FullExplore, WarnMinimal],
            },
            DetoxPhaseProfile {
                phase: AcuteWithdrawal,
                roh_band: Warning,
                k_min: 0.88,
                k_max: 0.94,
                d_min: 0.35,
                d_max: 0.55,
                dw_min: 0.14,
                dw_max: 0.20,
                max_dw_soft: 0.18,
                max_dw_hard: 0.20,
                allowed_modes: vec![WarnMinimal, HardStopExplain],
            },
            DetoxPhaseProfile {
                phase: EarlyRecovery,
                roh_band: Calm, // edge Calm/Warning handled via D,DW
                k_min: 0.90,
                k_max: 0.95,
                d_min: 0.25,
                d_max: 0.40,
                dw_min: 0.12,
                dw_max: 0.18,
                max_dw_soft: 0.18,
                max_dw_hard: 0.20,
                allowed_modes: vec![FullExplore, WarnMinimal],
            },
            DetoxPhaseProfile {
                phase: NeuroadaptationRev,
                roh_band: Calm,
                k_min: 0.92,
                k_max: 0.96,
                d_min: 0.18,
                d_max: 0.30,
                dw_min: 0.10,
                dw_max: 0.16,
                max_dw_soft: 0.18,
                max_dw_hard: 0.20,
                allowed_modes: vec![FullExplore, WarnMinimal],
            },
            DetoxPhaseProfile {
                phase: StableRecovery,
                roh_band: Calm,
                k_min: 0.92,
                k_max: 0.97,
                d_min: 0.10,
                d_max: 0.22,
                dw_min: 0.08,
                dw_max: 0.14,
                max_dw_soft: 0.18,
                max_dw_hard: 0.20,
                allowed_modes: vec![FullExplore, WarnMinimal],
            },
        ];

        Self {
            id: id.to_string(),
            profiles,
        }
    }

    pub fn profile_for_phase(&self, phase: &DetoxPhase) -> Option<&DetoxPhaseProfile> {
        self.profiles.iter().find(|p| &p.phase == phase)
    }
}
