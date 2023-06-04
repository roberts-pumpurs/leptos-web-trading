use http::StatusCode;
use leptos::*;

use leptos_meta::*;

use crate::{components::Navbar, error_template::AppError, layout::DefaultLayout};

#[component]
pub fn CommunityPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Community"/>
        <DefaultLayout>
            <div>"Community page"</div>
        </DefaultLayout>
    }
}
