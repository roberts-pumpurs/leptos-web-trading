mod community;
mod error;
mod home;
mod market;

pub use community::*;
pub use error::*;
pub use home::*;
pub use market::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    market::register_server_functions();
}
