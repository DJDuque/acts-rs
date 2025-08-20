pub use ffi::*;

#[cxx::bridge]
mod ffi {
    #[namespace = "acts_sys::ffi"]
    #[derive(Debug)]
    #[repr(u32)]
    enum CuboidVolumeBoundsFace {
        NegativeZFace,
        PositiveZFace,
        NegativeXFace,
        PositiveXFace,
        NegativeYFace,
        PositiveYFace,
    }

    unsafe extern "C++" {
        include!("Acts/Geometry/CuboidVolumeBounds.hpp");
        include!("acts-sys/include/geometry/cuboid_volume_bounds.hpp");

        #[namespace = "acts_sys::ffi"]
        type CuboidVolumeBoundsFace;
    }
}
