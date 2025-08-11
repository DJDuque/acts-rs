pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    #[derive(Debug)]
    #[repr(i32)]
    enum MaterialUpdateStage {
        PreUpdate = -1,
        FullUpdate,
        PostUpdate,
    }

    #[derive(Debug)]
    #[repr(i32)]
    enum NoiseUpdateMode {
        removeNoise = -1,
        addNoise = 1,
    }

    // Don't derive `Debug` because different variants with the same
    // discriminant will show the same value (which is just confusing).
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

    unsafe extern "C++" {
        include!("Acts/Definitions/Common.hpp");

        type MaterialUpdateStage;
        type NoiseUpdateMode;
        type CoordinateIndices;
    }
}
