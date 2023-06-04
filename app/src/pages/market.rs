use leptos::*;
use leptos_meta::*;

use crate::components::markets::Markets;
use crate::layout::DefaultLayout;

#[component(transparent)]
pub fn MarketPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Markets"/>
        <DefaultLayout>
            <Markets/>
        </DefaultLayout>
    }
}
