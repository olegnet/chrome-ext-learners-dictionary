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
use futures_util::StreamExt;

use crate::ui::dictionaryLookup;

#[component]
pub(crate) fn SearchForm(
    search_str: Signal<String>,
) -> Element {
    let dictionary_lookup = use_coroutine(|mut rx| async move {
        while let Some(search) = rx.next().await {
            dictionaryLookup(search).await;
        }
    });

    rsx! {
        div { class: class!(text_sm),
            margin_top: "5px",
            form {
                action: "",
                onsubmit: move |event| {
                    event.stop_propagation();
                    dictionary_lookup.send(search_str());
                    search_str.set("".to_string());
                },
                label {
                    r#for: "search-text",
                    input { class: class!(outline),
                        oninput: move |event| search_str.set(event.value()),
                        placeholder: "search",
                        r#type: "text",
                        id: "search-text",
                        value: "{search_str}"
                    }
                }
                label {
                    margin_left: "5px",
                    title: "Search",
                    button { class: class!(btn btn_sm btn_outline),
                        "Search"
                    }
                }
            }
        }
    }
}
