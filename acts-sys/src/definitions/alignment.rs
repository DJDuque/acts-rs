pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    #[derive(Debug)]
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

    unsafe extern "C++" {
        include!("Acts/Definitions/Alignment.hpp");

        type AlignmentIndices;
        type AlignmentVector;
        type AlignmentRowVector;
        type AlignmentMatrix;
        type AlignmentToPositionMatrix;
        type AlignmentToBoundMatrix;
        type AlignmentToPathMatrix;
    }
}
