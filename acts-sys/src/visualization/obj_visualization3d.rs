pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Visualization/ObjVisualization3D.hpp");
        include!("acts-sys/include/helpers.hpp");

        type ObjVisualization3D;
        type IVisualization3D = crate::visualization::i_visualization3d::IVisualization3D;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_obj_visualization3d(precision: u32, scale: f64) -> UniquePtr<ObjVisualization3D>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_obj_visualization3d(
            obj: UniquePtr<ObjVisualization3D>,
        ) -> UniquePtr<IVisualization3D>;
    }
}
