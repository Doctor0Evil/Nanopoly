#[derive(Clone, Debug)]
pub struct VertexNm {
    pub x_nm: f64,
    pub y_nm: f64,
    pub z_nm: f64,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub start_index: usize,
    pub end_index: usize,
}

#[derive(Clone, Debug)]
pub enum SurfaceCharge {
    Negative,
    Neutral,
    Positive,
}

#[derive(Clone, Debug)]
pub enum BioAffinityTarget {
    NeuralMembrane,
    GlialCell,
    EndothelialCell,
    MuscleFiber,
    ExtracellularMatrix,
}

#[derive(Clone, Debug)]
pub struct BiophysicalMetadata {
    pub target: BioAffinityTarget,
    pub surface_charge: SurfaceCharge,
    pub hydrophobicity_index: f32, // 0.0 (hydrophilic) – 1.0 (hydrophobic)
    pub elastic_modulus_kpa: f32,  // stiffness range in kPa
}

#[derive(Clone, Debug)]
pub struct Nanopolygon {
    pub id: String,
    pub vertices: Vec<VertexNm>,
    pub edges: Vec<Edge>,
    pub surface_area_nm2: f64,
    pub mean_curvature: f64,
    pub bio: BiophysicalMetadata,
}

impl Nanopolygon {
    pub fn new(
        id: &str,
        vertices: Vec<VertexNm>,
        edges: Vec<Edge>,
        bio: BiophysicalMetadata,
    ) -> Self {
        let (area, curvature) = Self::compute_geometry(&vertices, &edges);
        Self {
            id: id.to_string(),
            vertices,
            edges,
            surface_area_nm2: area,
            mean_curvature: curvature,
            bio,
        }
    }

    fn compute_geometry(
        vertices: &Vec<VertexNm>,
        edges: &Vec<Edge>,
    ) -> (f64, f64) {
        // Placeholder for real geometry calculations in the XR lab-grid.
        // Hook this into your nanoswarm pipeline once measurement code is ready.
        let area = 0.0_f64;
        let curvature = 0.0_f64;
        (area, curvature)
    }
}
