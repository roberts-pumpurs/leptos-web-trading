use leptos::*;
use leptos_meta::*;

use crate::components::{Footer, Navbar};

#[component]
pub fn Base(cx: Scope, children: Children) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Navbar/>
        {children(cx)}
        <Footer/>
    }
}
