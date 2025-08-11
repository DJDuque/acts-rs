pub mod algebra;
pub mod alignment;
pub mod common;
pub mod direction;
pub mod particle_data;
pub mod pdg_particle;
pub mod tolerance;

/*
#[cxx::bridge(namespace = "Acts")]
mod ffi {
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
        include!("Acts/Definitions/TrackParametrization.hpp");

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
