pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    #[derive(Debug)]
    #[repr(i32)]
    enum AxisDirection {
        AxisX,
        AxisY,
        AxisZ,
        AxisR,
        AxisPhi,
        AxisRPhi,
        AxisTheta,
        AxisEta,
        AxisMag,
    }

    unsafe extern "C++" {
        include!("Acts/Utilities/AxisDefinitions.hpp");

        type AxisDirection;
    }
}
