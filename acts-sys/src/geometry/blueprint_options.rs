pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/BlueprintOptions.hpp");
        include!("acts-sys/include/helpers.hpp");

        type BlueprintOptions;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_blueprint_options() -> UniquePtr<BlueprintOptions>;
    }
}
