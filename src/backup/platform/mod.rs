#[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))]
pub mod no_runtime;
#[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))]
pub use no_runtime::*;

#[cfg(feature = "rialight_default_export")]
pub mod tokio_runtime;
#[cfg(feature = "rialight_default_export")]
pub use tokio_runtime::*;

#[cfg(feature = "rialight_browser_export")]
pub mod browser_runtime;
#[cfg(feature = "rialight_browser_export")]
pub use browser_runtime::*;