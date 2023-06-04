pub mod footer;
pub mod game_list;
pub mod home;
pub mod markets;
pub mod navbar;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    markets::register_server_functions();
}
