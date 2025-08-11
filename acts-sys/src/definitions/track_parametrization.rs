pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    #[derive(Debug)]
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

    #[derive(Debug)]
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
