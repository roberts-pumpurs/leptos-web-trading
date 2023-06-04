use http::StatusCode;
use leptos::*;

use leptos_meta::*;

use crate::{components::Navbar, error_template::AppError, layout::DefaultLayout};

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
