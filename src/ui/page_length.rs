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
#![allow(non_upper_case_globals)]

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;

const msg_unlimited_page_length: &str = "unlimited";
const msg_page_length_updated_to_unlimited: &str = "page length updated to unlimited";
const msg_page_length_updated_to_number: fn(&u32) -> String =
    |val| format!("page length updated to {}", &val);
const button_change: &str = "Change";

#[component]
pub(crate)
fn PageLength(
    placeholder_str: &'static str,
    page_length: Signal<Option<u32>>,
    offset: Signal<Option<u32>>,
) -> Element {
    let get_page_length = move || match page_length() {
        None => msg_unlimited_page_length.to_string(),
        Some(val) => val.to_string(),
    };
    let mut page_length_val = use_signal(|| get_page_length());
    let mut message_str = use_signal(|| String::new());

    rsx! {
        div {
            form { action: "",
                onsubmit: move |event| {
                    event.stop_propagation();
                    match page_length_val().trim().parse() {
                        Err(_) | Ok(0) => {
                            page_length.set(None);
                            message_str.set(msg_page_length_updated_to_unlimited.to_string());
                        }
                        Ok(val) => {
                            page_length.set(Some(val));
                            message_str.set(msg_page_length_updated_to_number(&val));
                        }
                    }
                    offset.set(None);
                    page_length_val.set(get_page_length());
                },
                input { class: class!(outline),
                    margin_top: "5px",
                    oninput: move |event| page_length_val.set(event.value()),
                    placeholder: "{placeholder_str}",
                    r#type: "text",
                    value: "{page_length_val}"
                }
                button { class: class!(btn btn_sm btn_outline),
                    margin_left: "5px",
                    "{button_change}"
                }
                p { class: class!(text_xs),
                    "{message_str}"
                }
            }
        }
    }
}
