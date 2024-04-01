use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::components::{atoms::logo::Logo, router::Route};

#[function_component(Header)] 
pub fn header() -> Html {
    let show_logout = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let onclick = {
        let show_logout = show_logout.clone();
        Callback::from(move |e: MouseEvent | {
            e.prevent_default();
            show_logout.set(!(*show_logout));
        })
    };
    let logout = {
        Callback::from(move |e: MouseEvent|  {
            e.prevent_default();
            let navigator = navigator.clone();
            spawn_local(async move {
                match Request::post("/api/auth/sign-out").send().await {
                    Ok(_) => {
                        navigator.replace(&Route::Login);
                    }
                    Err(e) => {
                        log_1(&e.to_string().into());
                    }
                }
            });
        })
    };
    html!{
        <nav class="bg-white border-gray-200 dark:bg-gray-900">
            <div class="flex flex-wrap justify-between mx-auto p-4">
            <Logo />
            <ul class="flex flex-col p-4 md:p-0 mt-4 font-medium border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                <li>
                    <button {onclick} id="dropdownNavbarLink"  class="items-center flex py-2 px-3 rounded dark:hover:text-blue-500 dark:text-white aria-selected:dark:text-blue-500 dark:border-gray-700" aria-controls="tab-content">{"Account"}
                        <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                        </svg>
                    </button>
                    if *show_logout {
                        <div id="dropdownNavbar" class="z-10 font-normal bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600">
                            <div class="py-">
                                <a onclick={logout} class="block cursor-pointer px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white">{"Sign out"}</a>
                            </div>
                        </div>
                    }
                </li>
            </ul>
        </div>
        </nav>
    }
}
