pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Definitions/ParticleData.hpp");

        type PdgParticle = crate::definitions::pdg_particle::PdgParticle;
        type ParticleData;

        #[rust_name = "find_charge_of_nucleus"]
        fn findChargeOfNucleus(pdg: PdgParticle) -> Result<f32>;

        #[rust_name = "find_mass_of_nucleus"]
        fn findMassOfNucleus(pdg: PdgParticle) -> Result<f32>;

        #[rust_name = "calculate_nucleus_mass"]
        fn calculateNucleusMass(pdg: PdgParticle) -> Result<f32>;
    }
}
