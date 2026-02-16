#[derive(Clone, Debug)]
pub enum ModuleClass {
    Clinical,
    Comfort,
    Maintenance,
    Performance,
    Entertainment,
    Monetization,
}

#[derive(Clone, Debug)]
pub struct UpgradeModule {
    pub id: String,
    pub label: String,
    pub required_citizen_stake: u64,
    pub delta_energy_d: f32,
    pub delta_dw: f32,
    pub module_class: ModuleClass,
}

impl UpgradeModule {
    pub fn allowed_for_host_type(&self, host_type: &HostType) -> bool {
        match host_type {
            HostType::Human => true,
            _ => !matches!(
                self.module_class,
                ModuleClass::Performance | ModuleClass::Entertainment | ModuleClass::Monetization
            ),
        }
    }
}
