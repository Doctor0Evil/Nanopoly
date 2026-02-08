#[derive(Clone, Debug)]
pub struct MemoryMetric {
    /// Knowledge clarity / usefulness, 0.0–1.0
    pub k: f32,
    /// Storage + retrieval energy demand (normalized), 0.0–1.0
    pub d: f32,
    /// Psych-risk / compliance leverage, 0.0–1.0
    pub dw: f32,
    /// Short, human-audit note
    pub notes: String,
}

impl MemoryMetric {
    pub fn new(k: f32, d: f32, dw: f32, notes: &str) -> Self {
        Self {
            k: k.clamp(0.0, 1.0),
            d: d.clamp(0.0, 1.0),
            dw: dw.clamp(0.0, 1.0),
            notes: notes.to_string(),
        }
    }
}
