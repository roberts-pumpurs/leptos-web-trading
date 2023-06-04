use leptos::*;
use leptos_meta::*;

use crate::layout::DefaultLayout;

#[component]
pub fn CommunityPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Community"/>
        <DefaultLayout>
            <div>"Community page"</div>
        </DefaultLayout>
    }
}
