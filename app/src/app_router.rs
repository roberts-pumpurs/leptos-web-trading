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
                            view! { cx,
                                <div>
                                    <div class="text-lg text-gray-700 flex justify-center mt-[2.5rem] sm:mt-[1.9rem] font-medium">
                                        "Select a market to start trading!"
                                    </div>
                                    <div class="flex justify-center">
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="mt-[-2rem] vsm:mt-[1rem] md:mt-[2rem] lg:mt-[5rem] h-[25rem] w-[25rem]"
                                            fill="#ababab"
                                            viewBox="0 0 465.793 465.793"
                                            xml:space="preserve"
                                        >
                                            <path d="m401.322 465.793 7.222-16.993H16.993V57.25L0 64.471 27.193 0l27.193 64.471-16.993-7.221V428.4h371.151l-7.222-16.993 64.471 27.193-64.471 27.193zM71.393 401.2h68v-81.6h-68v81.6zm170 0V244.8h-68v156.4h68zm47.6 0h68V176.8h-68v224.4zM69.102 221.259l4.583 19.883c73.896-17.054 185.13-66.47 261.746-150.076l25.473 18.965 9.689-82.831-76.574 33.021 24.976 18.604c-73.379 79.228-179.33 126.154-249.893 142.434z"></path>
                                        </svg>
                                    </div>
                                </div>
                            }
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
