#[derive(Clone, Debug)]
pub struct CaninePhysioSnapshot {
    pub thermal_distance_index: f32,   // TD, 0 best, 1 worst
    pub molecular_balance_index: f32,  // MB, 0 worst, 1 best
    pub fatigue_index: f32,            // 0 rested, 1 exhausted
    pub risk_score: f32,               // RoH proxy 0–1
    pub host_budget_utilization: f32,  // 0 low, 1 at ceiling
    pub eco_impact_score: f32,         // 0 green, 1 heavy
    pub healthy_engagement_band: f32,  // 0–1, calm engagement
    pub psych_risk_dw: f32,            // local DW 0–1
}

#[derive(Clone, Debug)]
pub struct CanineLifeforceIndex {
    pub value: f32, // 0–1
}

impl CanineLifeforceIndex {
    pub fn compute(s: &CaninePhysioSnapshot) -> Self {
        let bt = 1.0_f32 - s.thermal_distance_index.clamp(0.0, 1.0);
        let bm = s.molecular_balance_index.clamp(0.0, 1.0);
        let bf = 1.0_f32 - s.fatigue_index.clamp(0.0, 1.0);
        let br = 1.0_f32 - s.risk_score.clamp(0.0, 1.0);
        let bh = 1.0_f32 - s.host_budget_utilization.clamp(0.0, 1.0);
        let be = 1.0_f32 - s.eco_impact_score.clamp(0.0, 1.0);
        let bc = s.healthy_engagement_band.clamp(0.0, 1.0);

        let w_t = 0.20_f32;
        let w_m = 0.25_f32;
        let w_f = 0.20_f32;
        let w_r = 0.15_f32;
        let w_h = 0.10_f32;
        let w_e = 0.05_f32;
        let w_c = 0.05_f32;

        let base = w_t * bt + w_m * bm + w_f * bf + w_r * br + w_h * bh + w_e * be + w_c * bc;

        let dw_penalty_strength = 0.8_f32;
        let dw_penalty = dw_penalty_strength * s.psych_risk_dw.clamp(0.0, 1.0);

        let value = (base * (1.0_f32 - dw_penalty)).clamp(0.0, 1.0);
        Self { value }
    }

    pub fn is_below_critical(&self, critical_threshold: f32) -> bool {
        self.value < critical_threshold
    }
}
