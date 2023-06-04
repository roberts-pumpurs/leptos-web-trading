use leptos::*;

use leptos_meta::*;

use crate::layout::DefaultLayout;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Home"/>
        <DefaultLayout>
            <div>"Built using Rust"</div>
            <div>"Built using Leptos"</div>
            <div>"Built using Tailwind"</div>
        </DefaultLayout>
    }
}
