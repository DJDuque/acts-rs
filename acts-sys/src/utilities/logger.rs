pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Utilities/Logger.hpp");

        type Logger;

        #[rust_name = "get_dummy_logger"]
        fn getDummyLogger() -> &'static Logger;
    }
}
