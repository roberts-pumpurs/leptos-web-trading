use http::StatusCode;
use leptos::*;

use leptos_meta::*;

use crate::{components::Navbar, error_template::AppError, layout::DefaultLayout};

#[component]
pub fn MarketPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Market"/>
        <DefaultLayout>
            <div>"Market page"</div>
        </DefaultLayout>
    }
}
