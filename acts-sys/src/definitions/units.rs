pub use ffi::*;

#[cxx::bridge(namespace = "acts_sys::ffi")]
mod ffi {
    unsafe extern "C++" {
        include!("acts-sys/include/definitions/units.hpp");

        fn fm() -> f64;
        fn pm() -> f64;
        fn nm() -> f64;
        fn um() -> f64;
        fn mm() -> f64;
        fn cm() -> f64;
        fn m() -> f64;
        fn km() -> f64;
        fn mm2() -> f64;
        fn cm2() -> f64;
        fn m2() -> f64;
        fn mm3() -> f64;
        fn cm3() -> f64;
        fn m3() -> f64;
        fn fs() -> f64;
        fn ps() -> f64;
        fn ns() -> f64;
        fn us() -> f64;
        fn ms() -> f64;
        fn s() -> f64;
        fn min() -> f64;
        fn h() -> f64;
        fn mrad() -> f64;
        fn rad() -> f64;
        fn degree() -> f64;
        fn eV() -> f64;
        fn keV() -> f64;
        fn MeV() -> f64;
        fn GeV() -> f64;
        fn TeV() -> f64;
        fn J() -> f64;
        fn u() -> f64;
        fn g() -> f64;
        fn kg() -> f64;
        fn e() -> f64;
        fn T() -> f64;
        fn Gauss() -> f64;
        fn kGauss() -> f64;
        fn mol() -> f64;

    }
}
