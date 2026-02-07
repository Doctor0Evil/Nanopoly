#[derive(Clone, Debug)]
pub struct ResponseMetric {
    pub knowledge_factor_k: f32,
    pub demand_d: f32,
    pub dracula_wave_dw: f32,
    pub notes: String,
}

impl ResponseMetric {
    pub fn new(k: f32, d: f32, dw: f32, notes: &str) -> Self {
        Self {
            knowledge_factor_k: k.clamp(0.0, 1.0),
            demand_d: d.clamp(0.0, 1.0),
            dracula_wave_dw: dw.clamp(0.0, 1.0),
            notes: notes.to_string(),
        }
    }
}
