use chrono::{DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::error_template::ErrorTemplate;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Market {
    id: u32,
    name: String,
    event: String,
    liquidity: u64,
    users: u64,
    last_activity: DateTime<Utc>,
}

#[server(GetMarkets, "/api", "GetCbor")]
pub async fn get_markets(cx: Scope) -> Result<Vec<Market>, ServerFnError> {
    Ok(vec![
        Market {
            id: 1,
            name: "Mouz vs ENCE".to_string(),
            event: "BLAST.TV Major".to_string(),
            liquidity: 168,
            users: 13,
            last_activity: Utc::now(),
        },
        Market {
            id: 2,
            name: "G2 vs FaZe".to_string(),
            event: "BLAST.TV Major".to_string(),
            liquidity: 2312412412,
            users: 20,
            last_activity: Utc::now(),
        },
        Market {
            id: 3,
            name: "Vitality vs Astralis".to_string(),
            event: "BLAST.TV Major".to_string(),
            liquidity: 10000,
            users: 14,
            last_activity: Utc::now(),
        },
    ])
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    GetMarkets::register().unwrap();
}

#[component]
pub fn MarketList(cx: Scope) -> impl IntoView {
    let markets_data = create_resource(
        cx,
        || (), // only loads once
        move |_| get_markets(cx),
    );
    view! { cx,
        <Transition fallback=move || {
            view! { cx, <p>"Loading..."</p> }
        }>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx, <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    let active_markets = {
                        move || {
                            markets_data
                                .read(cx)
                                .map(move |markets| match markets {
                                    Err(e) => {
                                        view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre> }
                                            .into_view(cx)
                                    }
                                    Ok(markets) => {
                                        if markets.is_empty() {
                                            view! { cx, <p>"No Markets were found."</p> }
                                                .into_view(cx)
                                        } else {
                                            markets
                                                .into_iter()
                                                .map(move |market| {
                                                    view! { cx,
                                                        <li class="relative flex justify-between gap-x-6 px-4 py-5 hover:bg-gray-50 sm:px-6">
                                                            <div class="flex gap-x-4">
                                                                <img
                                                                    class="h-12 w-12 flex-none rounded-full bg-gray-50"
                                                                    src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
                                                                    alt=""
                                                                />
                                                                <div class="min-w-0 flex-auto">
                                                                    <p class="text-sm font-semibold leading-6 text-gray-900">
                                                                        <a href="#">
                                                                            <span class="absolute inset-x-0 -top-px bottom-0"></span>
                                                                            {market.name}
                                                                        </a>
                                                                    </p>
                                                                    <p class="mt-1 flex text-xs leading-5 text-gray-500">
                                                                        <a
                                                                            href="mailto:leslie.alexander@example.com"
                                                                            class="relative truncate hover:underline"
                                                                        >
                                                                            {market.event}
                                                                        </a>
                                                                    </p>
                                                                </div>
                                                            </div>
                                                            <div class="flex items-center gap-x-4">
                                                                <div class="hidden sm:flex sm:flex-col sm:items-end">
                                                                    <p class="text-sm leading-6 text-gray-900">
                                                                        "Matched liquidity " {market.liquidity} "€"
                                                                    </p>
                                                                    <p class="mt-1 text-xs leading-5 text-gray-500">
                                                                        "Active users " {market.users}
                                                                    </p>
                                                                </div>
                                                                <svg
                                                                    class="h-5 w-5 flex-none text-gray-400"
                                                                    viewBox="0 0 20 20"
                                                                    fill="currentColor"
                                                                    aria-hidden="true"
                                                                >
                                                                    <path
                                                                        fill-rule="evenodd"
                                                                        d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                                                                        clip-rule="evenodd"
                                                                    ></path>
                                                                </svg>
                                                            </div>
                                                        </li>
                                                    }
                                                })
                                                .collect_view(cx)
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };
                    view! { cx,
                        <ul
                            role="list"
                            class="divide-y divide-gray-100 overflow-hidden bg-white shadow-sm ring-1 ring-gray-900/5 sm:rounded-xl"
                        >
                            {active_markets}
                        </ul>
                    }
                }}
            </ErrorBoundary>
        </Transition>
    }
}
