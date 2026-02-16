use crate::nanopoly::canine_policy::{CanineApplication, CaninePolicyProfile, CanineHostBudget};
use crate::nanopoly::species::Species;

#[derive(Clone, Debug)]
pub enum AdvisoryMode {
    ExplainOnly,   // information only, no directives
    WarnMinimal,   // gentle alerts, no prescriptive control
    GuidedAssist,  // short, low-load prompts
    IntensiveCare, // reserved, time-limited clinical use
}

#[derive(Clone, Debug)]
pub struct CanineGovernanceLayer {
    pub id: String,
    pub species: Species,
    pub host_budget: CanineHostBudget,
    pub profiles: Vec<CaninePolicyProfile>,
}

impl CanineGovernanceLayer {
    pub fn new(id: &str, host_budget: CanineHostBudget, profiles: Vec<CaninePolicyProfile>) -> Self {
        Self {
            id: id.to_string(),
            species: Species::Canine,
            host_budget,
            profiles,
        }
    }

    fn find_profile(&self, application: &CanineApplication) -> Option<&CaninePolicyProfile> {
        self.profiles.iter().find(|p| &p.application == application)
    }

    pub fn recommend_mode(
        &self,
        application: &CanineApplication,
        d: f32,
        dw: f32,
        fatigue_index: f32,
    ) -> AdvisoryMode {
        let fi = fatigue_index.clamp(0.0, 1.0);
        let d_clamped = d.clamp(0.0, 1.0);
        let dw_clamped = dw.clamp(0.0, 1.0);

        if fi > 0.50 || d_clamped > self.host_budget.max_demand_ceiling {
            return AdvisoryMode::ExplainOnly;
        }

        if dw_clamped > self.host_budget.max_dw_ceiling {
            return AdvisoryMode::ExplainOnly;
        }

        if let Some(profile) = self.find_profile(application) {
            if d_clamped > profile.d_max || dw_clamped > profile.dw_max {
                return AdvisoryMode::ExplainOnly;
            }

            match application {
                CanineApplication::PassiveWelfareMonitoring => AdvisoryMode::WarnMinimal,
                CanineApplication::GuardianInteraction => AdvisoryMode::WarnMinimal,
                CanineApplication::EcologicalCohabitationResearch => AdvisoryMode::WarnMinimal,
                CanineApplication::VeterinaryNeurologySupport => {
                    if fi < 0.40 && d_clamped < (profile.d_max * 0.9) {
                        AdvisoryMode::GuidedAssist
                    } else {
                        AdvisoryMode::WarnMinimal
                    }
                }
                CanineApplication::EmergencyIntervention => AdvisoryMode::IntensiveCare,
            }
        } else {
            AdvisoryMode::ExplainOnly
        }
    }
}
