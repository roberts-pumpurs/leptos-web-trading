use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
pub mod error_template;

use components::{Footer, Header, Home};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Title text="Live Trading Example"/>
        <Router>
            <Header/>
            <div class="bg-white">
                <main>
                    <Routes>
                        <Route
                            path=""
                            view=|cx| {
                                view! { cx, <Home/> }
                            }
                        />
                    </Routes>
                </main>
            </div>
            <Footer/>
        </Router>
    }
}
