pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Definitions/Algebra.hpp");
        include!("acts-sys/include/definitions/algebra.hpp");

        type Transform3;

        #[namespace = "acts_sys::ffi"]
        fn make_transform3(
            rotation_x: f64,
            rotation_y: f64,
            rotation_z: f64,
            translation_x: f64,
            translation_y: f64,
            translation_z: f64,
        ) -> UniquePtr<Transform3>;
    }
}
