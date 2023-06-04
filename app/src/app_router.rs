use leptos::*;
use leptos_router::*;

use crate::components::markets::ladder_view::LadderView;
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::{CommunityPage, HomePage, MarketPage};

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors=outside_errors/> }
                .into_view(cx)
        }>
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
                            view! { cx, <LadderView/> }
                        }
                    />
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <div>"Select a market to start trading!"</div> }
                        }
                    />
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
