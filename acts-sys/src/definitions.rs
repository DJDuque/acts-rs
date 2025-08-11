pub mod algebra;

/*
#[cxx::bridge(namespace = "Acts")]
mod ffi {
    #[repr(u32)]
    enum AlignmentIndices {
        eAlignmentCenter0,
        eAlignmentCenter1,
        eAlignmentCenter2,
        eAlignmentRotation0,
        eAlignmentRotation1,
        eAlignmentRotation2,
        eAlignmentSize,
    }

    #[repr(i32)]
    enum MaterialUpdateStage {
        PreUpdate = -1,
        FullUpdate,
        PostUpdate,
    }

    #[repr(i32)]
    enum NoiseUpdateMode {
        removeNoise = -1,
        addNoise = 1,
    }

    #[repr(u32)]
    enum CoordinateIndices {
        ePos0 = 0,
        ePos1,
        ePos2,
        eTime,
        eMom0 = 0,
        eMom1,
        eMom2,
        eEnergy,
        eX = 0,
        eY,
        eZ,
    }

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
        include!("Acts/Definitions/Alignment.hpp");
        include!("Acts/Definitions/Common.hpp");
        include!("Acts/Definitions/Direction.hpp");
        include!("Acts/Definitions/ParticleData.hpp");
        include!("Acts/Definitions/PdgParticle.hpp");
        include!("Acts/Definitions/TrackParametrization.hpp");

        type AlignmentIndices;
        type AlignmentVector;
        type AlignmentRowVector;
        type AlignmentMatrix;
        type AlignmentToPositionMatrix;
        type AlignmentToBoundMatrix;
        type AlignmentToPathMatrix;

        type MaterialUpdateStage;
        type NoiseUpdateMode;
        type CoordinateIndices;

        type Direction;

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
