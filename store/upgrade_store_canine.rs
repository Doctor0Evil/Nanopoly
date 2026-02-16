use crate::nanopoly::species::Species;
use crate::nanopoly::governance_canine::CanineConsentState;
use crate::store::metrics::ResponseMetric;

#[derive(Clone, Debug)]
pub struct UpgradeModule {
    pub id: String,
    pub label: String,
    pub required_citizen_stake: u64,
    pub delta_energy_d: f32,
    pub delta_dw: f32,
    pub commercial_flag: bool,
}

#[derive(Clone, Debug)]
pub struct UpgradeDecision {
    pub species: Species,
    pub module_id: String,
    pub metric: ResponseMetric,
    pub allowed: bool,
    pub reason: String,
}

pub struct CanineUpgradeGate;

impl CanineUpgradeGate {
    pub fn evaluate(
        species: Species,
        consent: &CanineConsentState,
        lfi: f32,
        module: &UpgradeModule,
    ) -> UpgradeDecision {
        let mut allowed = true;
        let mut reason = String::new();

        if species == Species::Canine {
            if module.commercial_flag || consent.non_commercial_only == false {
                allowed = false;
                reason.push_str("Non-commercial-only violation; ");
            }
            if consent.no_forced_upgrade {
                // For canines, we never accept upgrades that raise DW above 0.10 or LFI drops.
                if module.delta_dw > 0.10_f32 {
                    allowed = false;
                    reason.push_str("Psych-risk DW above canine ceiling; ");
                }
            }
            if lfi < 0.4_f32 {
                allowed = false;
                reason.push_str("LifeforceIndex below critical threshold; ");
            }
            if !consent.is_operation_allowed() {
                allowed = false;
                reason.push_str("Guardian consent missing or locked; ");
            }
        }

        if allowed && reason.is_empty() {
            reason.push_str("Upgrade allowed under canine neurorights envelope.");
        }

        let metric = ResponseMetric::new(
            0.9_f32,
            module.delta_energy_d,
            module.delta_dw,
            reason.as_str(),
        );

        UpgradeDecision {
            species,
            module_id: module.id.clone(),
            metric,
            allowed,
            reason,
        }
    }
}
