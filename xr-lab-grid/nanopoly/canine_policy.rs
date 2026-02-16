use crate::nanopoly::species::Species;

#[derive(Clone, Debug)]
pub enum CanineApplication {
    VeterinaryNeurologySupport,
    PassiveWelfareMonitoring,
    EcologicalCohabitationResearch,
    GuardianInteraction,
    EmergencyIntervention,
}

#[derive(Clone, Debug)]
pub struct RohBand {
    pub roh_min: f32,
    pub roh_max: f32,
}

#[derive(Clone, Debug)]
pub struct CanineHostBudget {
    pub body_mass_kg: f32,
    pub age_years: f32,
    pub chronic_condition_score: f32, // 0 healthy, 1 severe
    pub max_demand_ceiling: f32,      // derived D ceiling
    pub max_dw_ceiling: f32,          // derived DW ceiling
}

impl CanineHostBudget {
    pub fn derive(body_mass_kg: f32, age_years: f32, chronic_condition_score: f32) -> Self {
        let age_factor = (1.0 - (age_years / 20.0).clamp(0.0, 0.8)) as f32;
        let condition_factor = (1.0 - chronic_condition_score.clamp(0.0, 0.7)) as f32;

        let base_d = 0.35_f32;   // conservative default from your table
        let base_dw = 0.10_f32;  // conservative psych-risk ceiling

        let max_demand_ceiling = (base_d * age_factor * condition_factor).clamp(0.15, base_d);
        let max_dw_ceiling = (base_dw * age_factor * condition_factor).clamp(0.05, base_dw);

        Self {
            body_mass_kg,
            age_years,
            chronic_condition_score,
            max_demand_ceiling,
            max_dw_ceiling,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CaninePolicyProfile {
    pub application: CanineApplication,
    pub roh_band: RohBand,
    pub k_min: f32,
    pub k_max: f32,
    pub d_min: f32,
    pub d_max: f32,
    pub dw_min: f32,
    pub dw_max: f32,
    pub notes: String,
    pub non_commercial_only: bool,
}
