#[derive(Clone, Debug)]
pub struct AnimalWelfareMetricBundle {
    pub tdi: f32,           // ThermalDistanceIndex 0 best, 1 worst
    pub mbi: f32,           // MolecularBalanceIndex 0 worst, 1 best
    pub fatigue_index: f32, // 0 rested, 1 exhausted
    pub risk_score: f32,    // 0 safe, 1 high risk
    pub eco_impact: f32,    // 0 low, 1 high
    pub lifeforce_index: f32,
}

impl AnimalWelfareMetricBundle {
    pub fn from_raw(
        tdi: f32,
        mbi: f32,
        fatigue: f32,
        risk: f32,
        eco: f32,
        dw: f32,
        w_t: f32,
        w_m: f32,
        w_f: f32,
        w_r: f32,
        w_h: f32,
        w_e: f32,
    ) -> Self {
        // HostBudget utilization (H) is folded into eco/risk at this layer for animals.
        let bt = 1.0 - tdi;      // closer to safe band
        let bm = mbi;            // higher is better
        let bf = 1.0 - fatigue;
        let br = 1.0 - risk;
        let bh = 1.0 - eco;      // low eco load
        let be = 1.0 - eco;

        let base = w_t * bt + w_m * bm + w_f * bf + w_r * br + w_h * bh + w_e * be;
        let dw_penalty = dw.clamp(0.0, 1.0);
        let lifeforce = (base * (1.0 - dw_penalty)).clamp(0.0, 1.0);

        Self {
            tdi: tdi.clamp(0.0, 1.0),
            mbi: mbi.clamp(0.0, 1.0),
            fatigue_index: fatigue.clamp(0.0, 1.0),
            risk_score: risk.clamp(0.0, 1.0),
            eco_impact: eco.clamp(0.0, 1.0),
            lifeforce_index: lifeforce,
        }
    }
}
