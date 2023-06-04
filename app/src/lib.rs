use leptos::*;
use leptos_meta::*;

mod app_router;
mod components;
pub mod error_template;
mod layout;
mod pages;

#[cfg(feature = "ssr")]
pub use components::register_server_functions;

use crate::app_router::AppRouter;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Meta name="description" content="Webapp demo with Leptos in Rust"/>
        <AppRouter/>
    }
}
