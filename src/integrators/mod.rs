/// The integrator implements the light transport algorithm that computes radiance arriving at the film plane from surfaces and participating media in the scene.
pub trait Integrator {
    fn  Render(&mut self) {}
    fn  Preprocess(&mut self) {}
}

/// A "magic value array" for pixel bounds which meaning, when encountered, is to process the entire image.
const ENTIRE_IMAGE_PIXEL_BOUNDS: [u32; 4] = [0, 0, 0, 0]; 

trait SamplerIntegrator: Integrator {
    fn  Li(&self) {} // (&self, &ray: RayDifferential, scene: SceneData, sampler: Sampler, depth: int)
}



/// The light sample strategy is the technique used for sampling light sources.
/// Options include:
/// - "uniform", which samples all light sources uniformly
/// - "power", which samples light sources according to their emitted power
/// - "spatial", which computes light contributions in regions of the scene and samples from a related distribution.
/// (cf. http://www.pbrt.org/fileformat-v3.html#integrators )
pub enum LightSampleStrategy {
    Uniform,
    Power,
    Spatial
}

/// Parameters of a Path integrator:
/// max_depth: Maximum length of a light-carrying path sampled by the integrator.
/// pixel_bounds: Subset of image to sample during rendering; values given specify the starting and ending x coordinates and then starting and ending y coordinates.
///     (This functionality is primarily useful for narrowing down to a few pixels for debugging.)
/// rr_threshold: Determines when Russian roulette is applied to paths: when the maximum spectral component of the path contribution falls beneath this value, Russian roulette starts to be used.
/// light_sample_strat: The technique used for sampling light sources.
/// (cf. http://www.pbrt.org/fileformat-v3.html#integrators )
pub struct PathIntegrator {
    max_depth: u32,
    pixel_bounds: [u32; 4],
    rr_threshold: f32,
    light_sample_strat: LightSampleStrategy
}


impl PathIntegrator {
    /// The provided default values match the ones defined in the original PBRT spec.
    pub fn new() -> Self {
        PathIntegrator{
            max_depth:          5,
            pixel_bounds:       ENTIRE_IMAGE_PIXEL_BOUNDS,
            rr_threshold:       1.0,
            light_sample_strat: LightSampleStrategy::Spatial
        }
    }

    pub fn with_max_depth(&mut self, max_depth: u32) -> &mut Self {
        self.max_depth = max_depth;
        self
    }

    // pub fn new(max_depth: Option<u32>, pixel_bounds: Option<&[u32; 4]>, rr_threshold: Option<f32>, light_sample_strat: Option<LightSampleStrategy>) -> Self {
    //     PathIntegrator{
    //         max_depth:          max_depth.unwrap_or(5),
    //         pixel_bounds:       *pixel_bounds.unwrap_or(&ENTIRE_IMAGE_PIXEL_BOUNDS),
    //         rr_threshold:       rr_threshold.unwrap_or(1.0),
    //         light_sample_strat: light_sample_strat.unwrap_or(LightSampleStrategy::Spatial)
    //     }
    // }    
}

impl Integrator for PathIntegrator {

}

impl SamplerIntegrator for PathIntegrator {

}