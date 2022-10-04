#[cfg(feature = "soft")]
pub type F64 = softfp::F64;

#[cfg(not(feature = "soft"))]
pub type F64 = f64;