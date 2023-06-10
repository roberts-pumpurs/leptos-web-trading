pub mod ladder_view;
pub mod market_list;

use leptos::*;
use leptos_router::Outlet;
use market_list::MarketList;

#[component]
pub fn Markets(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex min-h-full flex-col">
            <div class="mx-auto flex w-full max-w-7xl items-start gap-x-8 px-4 py-10 sm:px-6 lg:px-8">
                <aside class="sticky top-8 hidden shrink-0 lg:block">
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
