pub mod algebra;
pub mod alignment;
pub mod common;
pub mod direction;

/*
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

    #[repr(u32)]
    enum BoundIndices {
        eBoundLoc0,
        eBoundLoc1,
        eBoundPhi,
        eBoundTheta,
        eBoundQOverP,
        eBoundTime,
        eBoundSize,
    }

    #[repr(u32)]
    enum FreeIndices {
        eFreePos0,
        eFreePos1,
        eFreePos2,
        eFreeTime,
        eFreeDir0,
        eFreeDir1,
        eFreeDir2,
        eFreeQOverP,
        eFreeSize,
    }

    unsafe extern "C++" {
        include!("Acts/Definitions/ParticleData.hpp");
        include!("Acts/Definitions/PdgParticle.hpp");
        include!("Acts/Definitions/TrackParametrization.hpp");

        type ParticleData;

        type PdgParticle;

        type BoundIndices;
        type FreeIndices;
        type BoundVector;
        type BoundMatrix;
        type BoundSquareMatrix;
        type BoundToFreeMatrix;
        type FreeVector;
        type FreeMatrix;
        type FreeSquareMatrix;
        type FreeToBoundMatrix;
        type FreeToPathMatrix;
    }
}
*/
