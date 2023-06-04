use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
pub mod error_template;
mod layout;
mod pages;

use pages::{MarketPage, CommunityPage, HomePage};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=|cx| {
                        view! { cx, <HomePage/> }
                    }
                />
                <Route
                    path="/market"
                    view=|cx| {
                        view! { cx, <MarketPage/> }
                    }
                />
                <Route
                    path="/community"
                    view=|cx| {
                        view! { cx, <CommunityPage/> }
                    }
                />
            </Routes>
        </Router>
    }
}
