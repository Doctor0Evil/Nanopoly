use crate::nanopoly::species::Species;

#[derive(Clone, Debug)]
pub enum GuardianRole {
    AccreditedGuardian,
    LicensedVeterinarian,
}

#[derive(Clone, Debug)]
pub struct GuardianConsent {
    pub guardian_id: String,        // DID or ALN identity
    pub role: GuardianRole,
    pub signed_at_unix: i64,
    pub expires_at_unix: i64,
    pub notes: String,
}

#[derive(Clone, Debug)]
pub struct CanineConsentState {
    pub species: Species,
    pub guardian: Option<GuardianConsent>,
    pub locked: bool,
    pub non_commercial_only: bool,
    pub no_forced_upgrade: bool,
}

impl CanineConsentState {
    pub fn new_canine() -> Self {
        Self {
            species: Species::Canine,
            guardian: None,
            locked: true,
            non_commercial_only: true,
            no_forced_upgrade: true,
        }
    }

    pub fn attach_guardian(&mut self, guardian: GuardianConsent, now_unix: i64) {
        // Consent only unlocks inside validity window.
        let valid = guardian.expires_at_unix > now_unix;
        self.guardian = Some(guardian);
        self.locked = !valid;
    }

    pub fn is_operation_allowed(&self) -> bool {
        self.species == Species::Canine
            && self.non_commercial_only
            && self.no_forced_upgrade
            && self.guardian.is_some()
            && !self.locked
    }
}
