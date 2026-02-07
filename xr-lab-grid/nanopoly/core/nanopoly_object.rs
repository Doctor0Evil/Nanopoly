#[derive(Clone, Debug)]
pub struct NanoPolygon {
    pub vertices_nm: Vec<[f64; 3]>,     // nanometer coordinates
    pub surface_area_nm2: f64,
    pub curvature_signature: Vec<f64>,  // compressed shape invariant
}

#[derive(Clone, Debug)]
pub enum BioAttachmentMode {
    NeuralSynaptic,
    NeuralGlial,
    VascularEndothelial,
    MuscularFiber,
    ExtracellularMatrix,
}

#[derive(Clone, Debug)]
pub struct EnergeticProfile {
    pub basal_glucose_uW: f64,   // microwatt equivalent
    pub peak_glucose_uW: f64,
    pub protein_turnover_uW: f64,
    pub hemodynamic_coupling: f64, // 0–1 coupling to blood flow
}

#[derive(Clone, Debug)]
pub struct BciInterface {
    pub input_bands_hz: Vec<(f32, f32)>,  // e.g. (8.0, 12.0) for alpha
    pub output_bands_hz: Vec<(f32, f32)>,
    pub max_bit_rate_bps: f32,
}

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
    pub min_citizen_stake: u64,      // CITIZEN token threshold
    pub upgrade_revision: u64,
}

#[derive(Clone, Debug)]
pub struct NanopolyObject {
    pub id: String,
    pub polygon: NanoPolygon,
    pub attachment: BioAttachmentMode,
    pub energy: EnergeticProfile,
    pub bci: BciInterface,
    pub gov: GovernanceLayer,
}
