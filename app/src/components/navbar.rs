use leptos::*;
use leptos_router::A;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    let (show_dropdown, set_show_dropdown) = create_signal(cx, false);

    view! { cx,
        <header class="inset-x-0 top-0 z-50">
            <nav class="flex items-center justify-between p-6 lg:px-8" aria-label="Global">
                <div class="flex lg:flex-1">
                    <A href="/" class="-m-1.5 p-1.5">
                        <span class="sr-only">"Your Company"</span>
                        <img
                            class="h-8 w-auto"
                            src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                            alt=""
                        />
                    </A>
                </div>
                <div class=move || if show_dropdown.get() { "hidden lg:hidden" } else { "flex lg:hidden" }>
                    <button
                        type="button"
                        class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700"
                        on:click=move |_| set_show_dropdown(true)
                    >
                        <span class="sr-only">"Open main menu"</span>
                        <svg
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            aria-hidden="true"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                            ></path>
                        </svg>
                    </button>
                </div>
                <div class="hidden lg:flex lg:gap-x-12">
                    <A href="/" class="text-sm font-semibold leading-6 text-gray-900">
                        "Home"
                    </A>
                    <A href="/market" class="text-sm font-semibold leading-6 text-gray-900">
                        "Markets"
                    </A>
                    <A href="/community" class="text-sm font-semibold leading-6 text-gray-900">
                        "Community"
                    </A>
                </div>
            </nav>
            <div
                class=move || if show_dropdown.get() { "lg:hidden" } else { "hidden lg:hidden" }
                role="dialog"
                aria-modal="true"
            >
                <div class="fixed inset-0 z-50"></div>
                <div class="fixed inset-y-0 right-0 z-50 w-full overflow-y-auto bg-white px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10">
                    <div class="flex items-center justify-between">
                        <A href="#" class="-m-1.5 p-1.5">
                            <span class="sr-only">"Your Company"</span>
                            <img
                                class="h-8 w-auto"
                                src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                                alt=""
                            />
                        </A>
                        <button
                            type="button"
                            class="-m-2.5 rounded-md p-2.5 text-gray-700"
                            on:click=move |_| set_show_dropdown(false)
                        >
                            <span class="sr-only">"Close menu"</span>
                            <svg
                                class="h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M6 18L18 6M6 6l12 12"
                                ></path>
                            </svg>
                        </button>
                    </div>
                    <div class="mt-6 flow-root">
                        <div class="-my-6 divide-y divide-gray-500/10">
                            <div class="space-y-2 py-6">
                                <A
                                    href="/"
                                    class="-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50"
                                >
                                    "Home"
                                </A>
                                <A
                                    href="/market"
                                    class="-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50"
                                >
                                    "Markets"
                                </A>
                                <A
                                    href="/community"
                                    class="-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50"
                                >
                                    "Community"
                                </A>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}
