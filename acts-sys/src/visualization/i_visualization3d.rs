pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Visualization/IVisualization3D.hpp");
        include!("acts-sys/include/visualization/i_visualization3d.hpp");

        type IVisualization3D;

        #[namespace = "acts_sys::ffi"]
        fn i_visualization3d_write(vis: &IVisualization3D, path: &CxxString);
    }

    impl UniquePtr<IVisualization3D> {}
}
