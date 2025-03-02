/*
 * Copyright (c) 2024-2025 Oleg Okhotnikov
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
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};
use crate::ui::AUTOPLAY;
use crate::ui::navigation::{DataProtection, NavigationState};
use crate::ui::page_length::PageLength;

#[component]
pub(crate) fn Settings(
    folders_page_length: Signal<Option<u32>>,
    folders_offset: Signal<Option<u32>>,
    words_page_length: Signal<Option<u32>>,
    words_offset: Signal<Option<u32>>,
    data_protection: Signal<DataProtection>,
) -> Element {
    let navigation = use_coroutine_handle::<NavigationState>();

    let data_protection_memo = use_memo(move || match data_protection() {
        DataProtection::Unprotected => "You can delete folders and words",
        DataProtection::Protected => "Uncheck to be able to delete folders and words",
    });

    let mut autoplay = use_synced_storage::<LocalStorage, bool>(
        "autoplay".to_string(), || true);

    let autoplay_memo = use_memo(
        move || if autoplay() { "Autoplay is enabled" } else { "Autoplay is disabled" });

    rsx! {
        div { class: class!(text_base),
            margin_top: "10px",
            div {
                button { class: class!(btn btn_sm),
                    onclick: move |_| navigation.send(NavigationState::ExportData),
                    "Export"
                }
                button { class: class!(btn btn_sm),
                    margin_left: "5px",
                    onclick: move |_| navigation.send(NavigationState::ImportData),
                    "Import"
                }
            }

            hr { margin_top: "10px" }
            div {
                margin_top: "10px",
                "Page length"
                div {
                    margin_top: "5px",
                    "Folders"
                    PageLength {
                        placeholder_str: "page length for folders",
                        page_length: folders_page_length,
                        offset: folders_offset,
                    }
                }
                div {
                    margin_top: "5px",
                    "Words"
                    PageLength {
                        placeholder_str: "page length for words",
                        page_length: words_page_length,
                        offset: words_offset,
                    }
                }
            }

            hr { margin_top: "10px" }
            div {
                margin_top: "10px",
                "Data protection"
                div {
                    margin_top: "5px",
                    form {
                        action: "",
                        onsubmit: move |event| event.stop_propagation(),
                        input {
                            r#type: "checkbox",
                            checked: data_protection() == DataProtection::Protected,
                            onchange: move |event| {
                                match event.checked() {
                                    true => data_protection.set(DataProtection::Protected),
                                    false => data_protection.set(DataProtection::Unprotected),
                                }
                            },
                        }
                        " {data_protection_memo}"
                    }
                }
            }

            hr { margin_top: "10px" }
            div {
                margin_top: "10px",
                "Autoplay"
                div {
                    margin_top: "5px",
                    form {
                        action: "",
                        onsubmit: move |event| event.stop_propagation(),
                        input {
                            r#type: "checkbox",
                            checked: autoplay(),
                            onchange: move |event| {
                                AUTOPLAY.with_mut(move |v| *v = event.checked());
                                autoplay.set(AUTOPLAY());
                            },
                        }
                        " {autoplay_memo}"
                    }
                }
            }

        }
    }
}
