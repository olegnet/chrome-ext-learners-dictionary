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

use crate::ui::navigation::Navigation;

// TODO Fonts sizes, dark theme
// FIXME add copyrights and fonts creds here

#[component]
pub(crate) fn Settings() -> Element {
    let navigation = use_coroutine_handle::<Navigation>();

    rsx! {
        div { class: class!(text_base),
            margin_top: "10px",
            div {
                button { class: class!(btn btn_sm),
                    onclick: move |_| navigation.send(Navigation::ExportData),
                    "Export"
                }
                button { class: class!(btn btn_sm),
                    margin_left: "5px",
                    onclick: move |_| navigation.send(Navigation::ImportData),
                    "Import"
                }
            }
        }
    }
}
