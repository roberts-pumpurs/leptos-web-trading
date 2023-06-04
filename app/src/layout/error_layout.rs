use leptos::*;
use leptos_meta::*;

#[component]
pub fn ErrorLayout(cx: Scope, children: Children) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Html class="h-full"/>
        <Body class="h-full"/>
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        {children(cx)}
    }
}
