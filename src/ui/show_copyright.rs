/*
 * Copyright (c) 2024 Oleg Okhotnikov
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

#[component]
pub fn ShowCopyright() -> Element {
    rsx! {
        div { class: class!(text_base),
            position: "absolute",
            bottom: "0",
            margin_bottom: "15px",
            p { "Copyright (c) 2024 Oleg Okhotnikov" }
            a { class: class!(text_sm link link_primary),
                href: "https://github.com/olegnet/chrome-ext-learners-dictionary",
                target: "_blank",
                "github.com/olegnet/chrome-ext-learners-dictionary"
            }
            hr { margin_top: "10px" }
            p { "All icons are from dioxus-free-icons project" }
            a { class: class!(text_sm link link_primary),
                href: "https://github.com/dioxus-community/dioxus-free-icons",
                target: "_blank",
                "github.com/dioxus-community/dioxus-free-icons"
            }
        }
    }
}
