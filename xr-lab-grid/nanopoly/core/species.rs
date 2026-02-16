#[derive(Clone, Debug)]
pub enum SpeciesId {
    HomoSapiens,
    CanisLupusFamiliaris,
    FelisCatus,
    CorvusCorax,
    Other(String),
}

#[derive(Clone, Debug)]
pub enum HostType {
    Human,
    NonHumanCompanion,
    WorkingAnimal,
    Wildlife,
    Other(String),
}

#[derive(Clone, Debug)]
pub struct SpeciesProfile {
    pub id: SpeciesId,
    // DEFAULTBIOPHYSEVIDENCE-style safe bands
    pub temp_low_c: f32,
    pub temp_high_c: f32,
    pub hr_rest_min: f32,
    pub hr_rest_max: f32,
    pub d_max_species: f32,   // 0–1 normalized D ceiling
    pub dw_max_species: f32,  // 0–1 normalized DW ceiling
    pub lifeforce_min_operational: f32,
}

#[derive(Clone, Debug)]
pub struct HostBudgetProfile {
    pub species: SpeciesId,
    pub host_type: HostType,
    pub d_max: f32,
    pub d_warn: f32,
    pub dw_max: f32,
    pub dw_warn: f32,
    pub duty_cycle_max: f32,   // fraction of time in active interface modes
}
