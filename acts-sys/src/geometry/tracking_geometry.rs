pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/TrackingGeometry.hpp");

        type TrackingGeometry;
    }

    impl UniquePtr<TrackingGeometry> {}
}
