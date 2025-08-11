pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    #[repr(i32)]
    enum PdgParticle {
        eInvalid,
        eElectron = 11,
        eAntiElectron = -11,
        ePositron = -11,
        eMuon = 13,
        eAntiMuon = -13,
        eTau = 15,
        eAntiTau = -15,
        eGamma = 22,
        ePionZero = 111,
        ePionPlus = 211,
        ePionMinus = -211,
        eKaonPlus = 321,
        eKaonMinus = -321,
        eNeutron = 2112,
        eAntiNeutron = -2112,
        eProton = 2212,
        eAntiProton = -2212,
        eLead = 1000822080,
    }

    unsafe extern "C++" {
        include!("Acts/Definitions/PdgParticle.hpp");

        type PdgParticle;

        #[rust_name = "make_absolute_pdg_particle"]
        fn makeAbsolutePdgParticle(pdg: PdgParticle) -> PdgParticle;

        #[rust_name = "is_nucleus"]
        fn isNucleus(pdg: PdgParticle) -> bool;

        #[rust_name = "make_nucleus_ground_state"]
        fn makeNucleusGroundState(p: PdgParticle) -> Result<PdgParticle>;
    }
}
