use leptos::*;
use leptos_meta::*;

use crate::layout::DefaultLayout;

#[component]
pub fn CommunityPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Community"/>
        <DefaultLayout>
            <div class="bg-white py-24 md:py-32">
                <div class="mx-auto grid max-w-7xl grid-cols-1 gap-x-8 gap-y-20 px-6 lg:px-8 xl:grid-cols-5">
                    <div class="max-w-2xl xl:col-span-2">
                        <h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                            "About the Speaker"
                        </h2>
                        <p class="mt-6 text-lg leading-8 text-gray-600">
                            "Roberts Ivanovs is a Rust Software Engineer. He has worked in the web3 space for the last 2 years, working on various projects during the NFT craze, marketplaces, DeFi. He has extensive experience using Rust for backend work and real-time system development, the IoT industry, web development, and Solidity/dApp development."
                        </p>
                    </div>
                    <ul
                        role="list"
                        class="-mt-12 space-y-12 divide-y divide-gray-200 xl:col-span-3"
                    >
                        <li class="flex flex-col gap-x-10 pt-12 sm:flex-row">
                            <img
                                class="aspect-[4/5] w-52 flex-none rounded-2xl object-cover"
                                src="/Roberts_Profile.png"
                                alt=""
                            />
                            <div class="max-w-xl mt-6 flex-auto">
                                <h3 class="text-xl font-semibold leading-8 tracking-tight text-gray-900">
                                    "Roberts Ivanovs"
                                </h3>
                                <p class="text-base leading-7 text-gray-600">
                                    "SIA ProvenCraft — Co-Founder / CEO"
                                </p>
                                <p class="mt-6 text-base leading-7 text-gray-600">
                                    "In betting, the only bet you're sure to win is the one you don't place."
                                </p>
                                <ul role="list" class="mt-6 flex gap-x-6">
                                    <li>
                                        <a
                                            href="https://github.com/roberts-ivanovs"
                                            class="text-gray-400 hover:text-gray-500"
                                        >
                                            <span class="sr-only">"Github"</span>
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                class="h-5 w-5 text-indigo-600"
                                                fill="currentColor"
                                                viewBox="0 0 24 24"
                                            >
                                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"></path>
                                            </svg>
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            href="https://www.linkedin.com/in/roberts-ivanovs-3b24b6159/"
                                            class="text-gray-400 hover:text-gray-500"
                                        >
                                            <span class="sr-only">"LinkedIn"</span>
                                            <svg
                                                class="h-5 w-5 text-indigo-600"
                                                fill="currentColor"
                                                viewBox="0 0 20 20"
                                                aria-hidden="true"
                                            >
                                                <path
                                                    fill-rule="evenodd"
                                                    d="M16.338 16.338H13.67V12.16c0-.995-.017-2.277-1.387-2.277-1.39 0-1.601 1.086-1.601 2.207v4.248H8.014v-8.59h2.559v1.174h.037c.356-.675 1.227-1.387 2.526-1.387 2.703 0 3.203 1.778 3.203 4.092v4.711zM5.005 6.575a1.548 1.548 0 11-.003-3.096 1.548 1.548 0 01.003 3.096zm-1.337 9.763H6.34v-8.59H3.667v8.59zM17.668 1H2.328C1.595 1 1 1.581 1 2.298v15.403C1 18.418 1.595 19 2.328 19h15.34c.734 0 1.332-.582 1.332-1.299V2.298C19 1.581 18.402 1 17.668 1z"
                                                    clip-rule="evenodd"
                                                ></path>
                                            </svg>
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </li>
                        <li class="flex flex-col gap-x-10 pt-12 sm:flex-row">
                            <img
                                class="h-48 w-52 flex-none rounded-2xl object-cover"
                                src="/PC_Logo.png"
                                alt=""
                            />
                            <div class="max-w-xl mt-6 sm:mt-0 flex-auto">
                                <h3 class="text-xl font-semibold leading-8 tracking-tight text-gray-900">
                                    "ProvenCraft Community"
                                </h3>
                                <p class="text-base leading-7 text-gray-600">"Tech Community"</p>
                                <p class="mt-6 text-base leading-7 text-gray-600">
                                    "A Safe and Inclusive Community for Your Personal and Professional Growth. At the core of our beliefs is the idea that we can achieve more together than alone. By coming together as a community, we can create a supportive and encouraging environment that allows individuals to learn from one another, grow personally and professionally, and achieve their goals."
                                </p>
                                <ul role="list" class="mt-6 flex gap-x-6">
                                    <li>
                                        <a
                                            href="https://community.provencraft.com/"
                                            class="text-gray-400 hover:text-gray-500"
                                        >
                                            <span class="sr-only">"Telegram"</span>
                                            <dt class="flex items-center text-indigo-600 gap-x-2 text-sm font-semibold leading-7">
                                                <svg
                                                    class="h-5 w-5 text-indigo-600"
                                                    aria-hidden="true"
                                                    fill="currentColor"
                                                    viewBox="0 0 24 24"
                                                    xmlns="http://www.w3.org/2000/svg"
                                                >
                                                    <path d="M21.721 12.752a9.711 9.711 0 00-.945-5.003 12.754 12.754 0 01-4.339 2.708 18.991 18.991 0 01-.214 4.772 17.165 17.165 0 005.498-2.477zM14.634 15.55a17.324 17.324 0 00.332-4.647c-.952.227-1.945.347-2.966.347-1.021 0-2.014-.12-2.966-.347a17.515 17.515 0 00.332 4.647 17.385 17.385 0 005.268 0zM9.772 17.119a18.963 18.963 0 004.456 0A17.182 17.182 0 0112 21.724a17.18 17.18 0 01-2.228-4.605zM7.777 15.23a18.87 18.87 0 01-.214-4.774 12.753 12.753 0 01-4.34-2.708 9.711 9.711 0 00-.944 5.004 17.165 17.165 0 005.498 2.477zM21.356 14.752a9.765 9.765 0 01-7.478 6.817 18.64 18.64 0 001.988-4.718 18.627 18.627 0 005.49-2.098zM2.644 14.752c1.682.971 3.53 1.688 5.49 2.099a18.64 18.64 0 001.988 4.718 9.765 9.765 0 01-7.478-6.816zM13.878 2.43a9.755 9.755 0 016.116 3.986 11.267 11.267 0 01-3.746 2.504 18.63 18.63 0 00-2.37-6.49zM12 2.276a17.152 17.152 0 012.805 7.121c-.897.23-1.837.353-2.805.353-.968 0-1.908-.122-2.805-.353A17.151 17.151 0 0112 2.276zM10.122 2.43a18.629 18.629 0 00-2.37 6.49 11.266 11.266 0 01-3.746-2.504 9.754 9.754 0 016.116-3.985z"></path>
                                                </svg>
                                                "Website Link"
                                            </dt>
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </li>
                        <li class="flex flex-col gap-x-10 pt-12 sm:flex-row">
                            <img
                                class="w-52 flex-none rounded-2xl object-cover"
                                src="/Telegram_QR.png"
                                alt=""
                            />
                            <div class="max-w-xl mt-6 flex-auto">
                                <h3 class="text-xl font-semibold leading-8 tracking-tight text-gray-900">
                                    "Personal Contact"
                                </h3>
                                <p class="text-base leading-7 text-gray-600">"@robertsivanovs"</p>
                                <p class="mt-6 text-base leading-7 text-gray-600">
                                    "Scan the QR Code to add me on telegram !"
                                </p>
                                <ul role="list" class="mt-6 flex gap-x-6">
                                    <li>
                                        <a
                                            href="https://t.me/robertsivanovs"
                                            class="text-gray-400 hover:text-gray-500"
                                        >
                                            <span class="sr-only">"Telegram"</span>
                                            <dt class="flex items-center text-indigo-600 gap-x-2 text-sm font-semibold leading-7">
                                                <svg
                                                    class="h-5 w-5 text-indigo-600"
                                                    fill="currentColor"
                                                    viewbox="0 0 24 24"
                                                    version="1.1"
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    xmlns:xlink="http://www.w3.org/1999/xlink"
                                                    xml:space="preserve"
                                                    xmlns:serif="http://www.serif.com/"
                                                    style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:1.41421;"
                                                >
                                                    <path
                                                        id="telegram-1"
                                                        d="M18.384,22.779c0.322,0.228 0.737,0.285 1.107,0.145c0.37,-0.141 0.642,-0.457 0.724,-0.84c0.869,-4.084 2.977,-14.421 3.768,-18.136c0.06,-0.28 -0.04,-0.571 -0.26,-0.758c-0.22,-0.187 -0.525,-0.241 -0.797,-0.14c-4.193,1.552 -17.106,6.397 -22.384,8.35c-0.335,0.124 -0.553,0.446 -0.542,0.799c0.012,0.354 0.25,0.661 0.593,0.764c2.367,0.708 5.474,1.693 5.474,1.693c0,0 1.452,4.385 2.209,6.615c0.095,0.28 0.314,0.5 0.603,0.576c0.288,0.075 0.596,-0.004 0.811,-0.207c1.216,-1.148 3.096,-2.923 3.096,-2.923c0,0 3.572,2.619 5.598,4.062Zm-11.01,-8.677l1.679,5.538l0.373,-3.507c0,0 6.487,-5.851 10.185,-9.186c0.108,-0.098 0.123,-0.262 0.033,-0.377c-0.089,-0.115 -0.253,-0.142 -0.376,-0.064c-4.286,2.737 -11.894,7.596 -11.894,7.596Z"
                                                    ></path>
                                                </svg>
                                                "Telegram Link"
                                            </dt>
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </li>
                    </ul>
                </div>
            </div>
        </DefaultLayout>
    }
}
