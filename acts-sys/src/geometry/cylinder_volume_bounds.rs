pub use ffi::*;

#[cxx::bridge]
mod ffi {
    #[namespace = "acts_sys::ffi"]
    #[derive(Debug)]
    #[repr(u32)]
    enum CylinderVolumeBoundsFace {
        NegativeDisc,
        PositiveDisc,
        OuterCylinder,
        InnerCylinder,
        NegativePhiPlane,
        PositivePhiPlane,
    }

    unsafe extern "C++" {
        include!("Acts/Geometry/CylinderVolumeBounds.hpp");
        include!("acts-sys/include/geometry/cylinder_volume_bounds.hpp");

        #[namespace = "acts_sys::ffi"]
        type CylinderVolumeBoundsFace;
    }
}
