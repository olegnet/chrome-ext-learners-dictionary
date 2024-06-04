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
pub(crate) fn Settings(
    folders_page_length: Signal<Option<u32>>,
    page_length: Signal<Option<u32>>,
) -> Element {
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
            // FIXME texts
            div {
                margin_top: "10px",
                "folders_page_length:"
                PageLength {
                    placeholder_str: "folders_page_length",
                    page_length: folders_page_length
                }
            }
            div {
                margin_top: "10px",
                "page_length:"
                PageLength {
                    placeholder_str: "page_length",
                    page_length: page_length
                }
            }
        }
    }
}

const UNLIMITED: &str = "unlimited";

#[component]
pub(crate) fn PageLength(
    placeholder_str: &'static str,
    page_length: Signal<Option<u32>>,
) -> Element {
    let get_page_length = move || match page_length() {
        None => String::from(UNLIMITED),
        Some(val) => val.to_string(),
    };
    let mut page_length_val = use_signal(|| get_page_length());

    // FIXME messages
    let mut message_str = use_signal(|| String::new());

    rsx! {
        div {
            form { action: "",
                onsubmit: move |event| {
                    event.stop_propagation();
                    match page_length_val().trim().parse() {
                        Err(_) | Ok(0) => {
                            page_length.set(None);
                            message_str.set("page length updated to unlimited".to_string());
                            page_length_val.set(get_page_length());
                        },
                        Ok(val) => {
                            page_length.set(Some(val));
                            message_str.set(format!("page length updated to {}", &val));
                            page_length_val.set(get_page_length());
                        },
                    }
                },
                label {
                    r#for: "folder_note",
                    input { class: class!(outline),
                        margin_top: "5px",
                        oninput: move |event| page_length_val.set(event.value()),
                        placeholder: "{placeholder_str}",
                        r#type: "text",
                        value: "{page_length_val}"
                    }
                }
                label {
                    title: "Change",
                    button { class: class!(btn btn_sm btn_outline),
                        margin_left: "5px",
                        "Change"
                    }
                }
                p { class: class!(text_xs),
                    "{message_str}"
                }
            }
        }
    }
}
