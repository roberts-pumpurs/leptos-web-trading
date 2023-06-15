pub mod ladder_view;
pub mod market_list;

use leptos::*;
use leptos_router::Outlet;
use market_list::MarketList;

#[component]
pub fn Markets(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex min-h-full flex-col">
            <div class="mx-auto lg:flex flex-col lg:flex-row w-full max-w-7xl items-start gap-x-8 px-3 py-2 lg:py-10 sm:px-6 lg:px-8">
                <aside class="mt-6 lg:mt-[2.7rem]">
                    <MarketList/>
                </aside>
                <main class="flex-1">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    market_list::register_server_functions();
}
