use leptos::*;
use leptos_meta::*;

use crate::layout::DefaultLayout;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Home"/>
        <DefaultLayout>
            <div class="bg-white py-24 sm:py-32">
                <div class="mx-auto max-w-7xl px-6 lg:px-8">
                    <div class="mx-auto max-w-2xl lg:text-center">
                        <p class="mt-2 text-3xl font-bold text-gray-700 sm:text-4xl">
                            "Leptos Demo Trading Platform"
                        </p>
                    </div>
                    <div class="mx-auto max-w-2xl lg:text-center">
                        <p class="mt-6 text-base leading-7 text-gray-600">
                            "In betting, the only bet you're sure to win is the one you don't place."
                        </p>
                    </div>
                    <div class="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-none">
                        <dl class="grid max-w-xl grid-cols-1 gap-x-10 gap-y-16 lg:max-w-none lg:grid-cols-3">
                            <div class="flex flex-col">
                                <img src="/Leptos_Logo.svg" alt="Leptos-Logo" class="w-26 sm:w-24"/>
                                <dt class="flex mt-8 sm:mt-5 items-center gap-x-3 text-2xl font-semibold leading-7 text-gray-700">
                                    <svg
                                        class="h-7 w-7 flex-none text-indigo-600"
                                        fill="currentColor"
                                        viewBox="0 0 24 24"
                                        xmlns="http://www.w3.org/2000/svg"
                                        aria-hidden="true"
                                    >
                                        <path
                                            clip-rule="evenodd"
                                            fill-rule="evenodd"
                                            d="M9.315 7.584C12.195 3.883 16.695 1.5 21.75 1.5a.75.75 0 01.75.75c0 5.056-2.383 9.555-6.084 12.436A6.75 6.75 0 019.75 22.5a.75.75 0 01-.75-.75v-4.131A15.838 15.838 0 016.382 15H2.25a.75.75 0 01-.75-.75 6.75 6.75 0 017.815-6.666zM15 6.75a2.25 2.25 0 100 4.5 2.25 2.25 0 000-4.5z"
                                        ></path>
                                        <path d="M5.26 17.242a.75.75 0 10-.897-1.203 5.243 5.243 0 00-2.05 5.022.75.75 0 00.625.627 5.243 5.243 0 005.022-2.051.75.75 0 10-1.202-.897 3.744 3.744 0 01-3.008 1.51c0-1.23.592-2.323 1.51-3.008z"></path>
                                    </svg>
                                    "Leptos"
                                </dt>
                                <dd class="mt-4 flex flex-auto flex-col text-lg leading-7 text-gray-600">
                                    <p class="flex-auto">"Performance and Reactivity of Leptos"</p>
                                    <p class="mt-6">
                                        <a
                                            href="https://leptos.dev/"
                                            class="text-sm font-semibold leading-6 text-indigo-600"
                                        >
                                            "Learn more "
                                            <span aria-hidden="true">"→"</span>
                                        </a>
                                    </p>
                                </dd>
                            </div>
                            <div class="flex flex-col">
                                <img src="/Ferris.svg" alt="Rust-Logo" class="w-26"/>
                                <dt class="flex mt-2 sm:mt-10 items-center gap-x-3 text-2xl font-semibold leading-7 text-gray-700">
                                    <svg
                                        class="h-7 w-7 flex-none text-indigo-600"
                                        fill="currentColor"
                                        viewBox="0 0 24 24"
                                        xmlns="http://www.w3.org/2000/svg"
                                        aria-hidden="true"
                                    >
                                        <path
                                            clip-rule="evenodd"
                                            fill-rule="evenodd"
                                            d="M12.516 2.17a.75.75 0 00-1.032 0 11.209 11.209 0 01-7.877 3.08.75.75 0 00-.722.515A12.74 12.74 0 002.25 9.75c0 5.942 4.064 10.933 9.563 12.348a.749.749 0 00.374 0c5.499-1.415 9.563-6.406 9.563-12.348 0-1.39-.223-2.73-.635-3.985a.75.75 0 00-.722-.516l-.143.001c-2.996 0-5.717-1.17-7.734-3.08zm3.094 8.016a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z"
                                        ></path>
                                    </svg>
                                    "Rust"
                                </dt>
                                <dd class="mt-4 flex flex-auto flex-col text-lg leading-7 text-gray-600">
                                    <p class="flex-auto">"Robustness and Type-safety of Rust"</p>
                                    <p class="mt-6">
                                        <a
                                            href="https://www.rust-lang.org/"
                                            class="text-sm font-semibold leading-6 text-indigo-600"
                                        >
                                            "Learn more "
                                            <span aria-hidden="true">"→"</span>
                                        </a>
                                    </p>
                                </dd>
                            </div>
                            <div class="flex flex-col mt-6">
                                <img
                                    src="/Tailwind_Logo.svg"
                                    alt="Tailwind-Logo"
                                    class="w-26 sm:w-25"
                                />
                                <dt class="flex mt-10 sm:mt-[4rem] items-center gap-x-3 text-2xl font-semibold leading-7 text-gray-700">
                                    <svg
                                        class="h-7 w-7 flex-none text-indigo-600"
                                        fill="currentColor"
                                        viewBox="0 0 24 24"
                                        xmlns="http://www.w3.org/2000/svg"
                                        aria-hidden="true"
                                    >
                                        <path
                                            clip-rule="evenodd"
                                            fill-rule="evenodd"
                                            d="M14.615 1.595a.75.75 0 01.359.852L12.982 9.75h7.268a.75.75 0 01.548 1.262l-10.5 11.25a.75.75 0 01-1.272-.71l1.992-7.302H3.75a.75.75 0 01-.548-1.262l10.5-11.25a.75.75 0 01.913-.143z"
                                        ></path>
                                    </svg>
                                    "Tailwind CSS"
                                </dt>
                                <dd class="mt-4 flex flex-auto flex-col text-lg leading-7 text-gray-600">
                                    <p class="flex-auto">
                                        "Creativity and Elegance of Tailwind CSS"
                                    </p>
                                    <p class="mt-6">
                                        <a
                                            href="https://tailwindcss.com/"
                                            class="text-sm font-semibold leading-6 text-indigo-600"
                                        >
                                            "Learn more "
                                            <span aria-hidden="true">"→"</span>
                                        </a>
                                    </p>
                                </dd>
                            </div>
                        </dl>
                    </div>
                </div>
            </div>
        </DefaultLayout>
    }
}
