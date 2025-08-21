pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Visualization/IVisualization3D.hpp");

        type IVisualization3D;
    }

    impl UniquePtr<IVisualization3D> {}
}
