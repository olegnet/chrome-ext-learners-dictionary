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
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fi_icons::{FiSearch, FiTrash};

use crate::model::{Word, WordKey};
use crate::ui::{SELECTED_WORD_INDEX, dictionaryLookup, openUrl};

#[component]
pub(crate) fn ShowWord(
    index: i32,
    word: ReadOnlySignal<Word>,
) -> Element {
    let word_key = use_coroutine_handle::<WordKey>();

    let selected_word_index = use_memo(move || SELECTED_WORD_INDEX());
    let is_selected = match selected_word_index() {
        None => false,
        Some(v) => index == v
    };

    if is_selected {
        spawn(openUrl(word().url.clone()));
    }

    let id = word().id.unwrap();
    let word_str = word().word;
    let word_class = word().word_class;
    let note = word().note;
    let url = word().url;

    let background_color = match index % 2 { 0 => "lists-second-colors", _ => "" };
    let selected_word = if is_selected { "lists-selected-colors" } else { "" };

    rsx! {
        div { class: class!(flex items_baseline background_color selected_word),
            onclick: move |_| {
                if let Some(_) = SELECTED_WORD_INDEX() {
                    *SELECTED_WORD_INDEX.write() = Some(index);
                }
            },
            div { class: class!(flex_none),
                a {
                    href: "#",
                    onclick: move |event| {
                        event.stop_propagation();
                        spawn(dictionaryLookup(word_str.clone()));
                    },
                    Icon {
                        height: 15,
                        width: 15,
                        icon: FiSearch,
                    }
                }
            }
            div { class: class!(flex_1),
                margin_left: "10px",
                a { class: class!(inline_block underline),
                    href: "#",
                    onclick: move |_| { spawn(openUrl(url.clone())); },
                    "{word_str}"
                }
                p { class: class!(text_xs),
                    "{word_class}"
                }
            }
            div { class: class!(flex_1),
                margin_left: "5px",
                "{note}"
            }
            div { class: class!(flex_none),
                a { class: class!(inline_block),
                    margin_right: "5px",
                    href: "#",
                    onclick: move |event| {
                        event.stop_propagation();
                        word_key.send(WordKey{ id });
                    },
                    Icon {
                        height: 15,
                        width: 15,
                        icon: FiTrash,
                    }
                }
            }
        }
    }
}
