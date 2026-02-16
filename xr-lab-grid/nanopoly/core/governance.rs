use crate::core::species::{HostType, SpeciesId};

#[derive(Clone, Debug)]
pub enum ConsentState {
    Locked,
    Active,
    Suspended,
}

#[derive(Clone, Debug)]
pub struct GovernanceLayer {
    pub owner_did: String,
    pub consent_state: ConsentState,
    pub min_citizen_stake: u64,
    pub upgrade_revision: u64,
    // New neurorights fields
    pub host_type: HostType,
    pub species: SpeciesId,
    pub guardian_did: Option<String>,
    pub veterinary_did: Option<String>,
    pub non_commercial: bool,
    pub no_entertainment: bool,
}
