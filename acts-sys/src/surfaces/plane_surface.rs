pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/PlaneSurface.hpp");

        type PlaneSurface;
    }
}
