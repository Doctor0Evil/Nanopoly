use crate::memory::metrics::MemoryMetric;

#[derive(Clone, Debug)]
pub enum MemoryKind {
    Preference,      // user likes/dislikes
    Fact,            // factual statement about user or world
    Goal,            // user goals / projects
    Constraint,      // privacy / boundaries
    ContextPattern,  // recurring patterns in conversations
}

#[derive(Clone, Debug)]
pub struct MemoryObject {
    /// Stable id for diffing and upgrades
    pub id: String,
    /// User id / DID
    pub owner_id: String,
    /// What type of memory this is
    pub kind: MemoryKind,
    /// Original natural-language form (compressed if needed)
    pub original_text: String,
    /// Canonical, condensed representation (e.g., key-value or schema form)
    pub canonical_form: String,
    /// Machine-usable vector / hash or other embedding id
    pub embedding_key: String,
    /// Strength + safety metrics
    pub metric: MemoryMetric,
    /// Monotonic revision (for persistence and rollbacks)
    pub revision: u64,
}
