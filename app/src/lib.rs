use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
pub mod error_template;
mod layout;
mod pages;

use components::Home;

use crate::layout::Base;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Base>
            <Router>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <Home/> }
                        }
                    />
                </Routes>
            </Router>
        </Base>
    }
}
