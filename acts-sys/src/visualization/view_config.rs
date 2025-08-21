pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Visualization/ViewConfig.hpp");
        include!("acts-sys/include/visualization/view_config.hpp");

        type ViewConfig;

        #[namespace = "acts_sys::ffi"]
        fn new_view_config(
            visible: bool,
            rgb_color: &[i32; 3],
            offset: f64,
            line_thickness: f64,
            surface_thickness: f64,
            quarter_segments: u32,
            triangulate: bool,
            output_name: &CxxString,
        ) -> UniquePtr<ViewConfig>;
    }
}
