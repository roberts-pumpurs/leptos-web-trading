use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
pub mod error_template;
mod layout;
mod pages;

#[cfg(feature = "ssr")]
pub use pages::register_server_functions;
use pages::{CommunityPage, HomePage, MarketPage,};

use crate::pages::LadderView;

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
                    >
                        <Route
                            path=":id"
                            view=|cx| {
                                view! { cx, <LadderView /> }
                            }
                        />
                        <Route path="" view=|cx| view! { cx,
                            <div>
                                "Select a market to start trading!"
                            </div>
                        }/>
                </Route>
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
