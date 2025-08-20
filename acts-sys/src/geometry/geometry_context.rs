pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/GeometryContext.hpp");
        include!("acts-sys/include/helpers.hpp");

        type GeometryContext;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_geometry_context() -> UniquePtr<GeometryContext>;
    }
}
