mod market_list;

use leptos::*;
use market_list::MarketList;

use leptos_meta::*;

use crate::layout::DefaultLayout;

#[component]
pub fn MarketPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Markets"/>
        <DefaultLayout>
            <MarketList/>
        </DefaultLayout>
    }
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    market_list::register_server_functions();
}
