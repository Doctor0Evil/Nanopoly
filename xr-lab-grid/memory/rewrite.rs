use crate::memory::metrics::MemoryMetric;
use crate::memory::object::{MemoryKind, MemoryObject};

#[derive(Clone, Debug)]
pub struct RewriteConfig {
    pub max_canonical_len: usize,
    pub max_original_len: usize,
    pub k_gain_weight: f32,
    pub d_penalty_weight: f32,
    pub dw_guard: f32,
}

impl Default for RewriteConfig {
    fn default() -> Self {
        Self {
            max_canonical_len: 256,
            max_original_len: 1024,
            k_gain_weight: 0.6,
            d_penalty_weight: 0.3,
            dw_guard: 0.25,
        }
    }
}

pub struct RewriteEngine {
    pub cfg: RewriteConfig,
}

impl RewriteEngine {
    pub fn new(cfg: RewriteConfig) -> Self {
        Self { cfg }
    }

    pub fn canonicalize(&self, mem: &MemoryObject) -> MemoryObject {
        // 1. Apply deterministic pattern rules, depending on MemoryKind.
        let canonical = self.canonical_from_text(&mem.original_text, &mem.kind);

        // 2. Truncate to resource bounds.
        let canonical = self.truncate_to_limit(&canonical, self.cfg.max_canonical_len);
        let original = self.truncate_to_limit(&mem.original_text, self.cfg.max_original_len);

        // 3. Recompute metric: increase K if canonical is more structured,
        // adjust D for shorter strings, keep DW <= guard.
        let (k, d, dw, notes) = self.estimate_metric(&original, &canonical, &mem.metric, &mem.kind);

        MemoryObject {
            id: mem.id.clone(),
            owner_id: mem.owner_id.clone(),
            kind: mem.kind.clone(),
            original_text: original,
            canonical_form: canonical,
            embedding_key: mem.embedding_key.clone(),
            metric: MemoryMetric::new(k, d, dw, &notes),
            revision: mem.revision + 1,
        }
    }

    fn canonical_from_text(&self, text: &str, kind: &MemoryKind) -> String {
        // Placeholder – plug in your own rule-based parser or on-device model.
        match kind {
            MemoryKind::Preference => format!("pref:{}", text.to_lowercase()),
            MemoryKind::Goal => format!("goal:{}", text.to_lowercase()),
            MemoryKind::Fact => format!("fact:{}", text.to_lowercase()),
            MemoryKind::Constraint => format!("constraint:{}", text.to_lowercase()),
            MemoryKind::ContextPattern => format!("pattern:{}", text.to_lowercase()),
        }
    }

    fn truncate_to_limit(&self, s: &str, limit: usize) -> String {
        if s.len() <= limit {
            s.to_string()
        } else {
            s.chars().take(limit).collect()
        }
    }

    fn estimate_metric(
        &self,
        original: &str,
        canonical: &str,
        old: &MemoryMetric,
        kind: &MemoryKind,
    ) -> (f32, f32, f32, String) {
        let compression = (canonical.len() as f32 + 1.0) / (original.len() as f32 + 1.0);
        let d = (old.d * 0.7 + compression * self.cfg.d_penalty_weight).clamp(0.0, 1.0);

        let structural_bonus = match kind {
            MemoryKind::Preference | MemoryKind::Constraint => 0.15,
            MemoryKind::Goal => 0.10,
            MemoryKind::Fact | MemoryKind::ContextPattern => 0.08,
        };

        let k = (old.k + structural_bonus).clamp(0.0, 1.0);

        let dw = old.dw.min(self.cfg.dw_guard);

        let notes = format!(
            "canonicalized; compression={:.3}; kind={:?}",
            compression, kind
        );

        (k, d, dw, notes)
    }
}
