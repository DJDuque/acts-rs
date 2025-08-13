pub use ffi::*;

#[cxx::bridge(namespace = "acts_sys::ffi")]
mod ffi {
    unsafe extern "C++" {
        include!("acts-sys/include/definitions/tolerance.hpp");

        #[rust_name = "epsilon"]
        fn s_epsilon() -> f64;

        #[rust_name = "on_surface_tolerance"]
        fn s_onSurfaceTolerance() -> f64;

        #[rust_name = "curvilinear_proj_tolerance"]
        fn s_curvilinearProjTolerance() -> f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epsilon() {
        let eps = epsilon();
        assert_eq!(eps, 6.661338147750939e-16);
    }

    #[test]
    fn test_on_surface_tolerance() {
        let tol = on_surface_tolerance();
        assert_eq!(tol, 1e-4);
    }

    #[test]
    fn test_curvilinear_proj_tolerance() {
        let tol = curvilinear_proj_tolerance();
        assert_eq!(tol, 0.999995);
    }
}
