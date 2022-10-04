mod soft;
mod hard;
#[cfg(feature = "softfp")]
pub use soft::F64;

#[cfg(not(feature = "softfp"))]
pub use hard::F64;