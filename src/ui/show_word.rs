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
use dioxus_sdk::storage::{LocalStorage, use_synced_storage};
use crate::model::{Word, WordKey};
use crate::ui::{SELECTED_WORD_INDEX, dictionaryLookup, openUrl, msg_use_arrow_keys_to_navigate};
use crate::ui::navigation::{NAVIGATION_MESSAGE_NOTIFICATION, NavigationMessage};

#[component]
pub(crate) fn ShowWord(
    index: i32,
    word: ReadOnlySignal<Word>,
) -> Element {
    let navigation_message = use_coroutine_handle::<NavigationMessage>();
    let mut show_use_keyboard_message = use_synced_storage::<LocalStorage, bool>(
        "show_use_keyboard_message".to_string(), || true);

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

    let background_color = if let 0 = index % 2 { "lists-second-style" } else { "" };
    let selected_word = if is_selected { "lists-selected-style" } else { "" };

    rsx! {
        div { class: class!(flex flex_wrap items_baseline min_h_12 background_color selected_word),
            id: "word-{index}",
            tabindex: "-1",
            margin: "1px",
            onclick: move |_| {
                *SELECTED_WORD_INDEX.write() = Some(index);
                if show_use_keyboard_message() {
                    show_use_keyboard_message.set(false);
                    navigation_message.send(NavigationMessage {
                        message: msg_use_arrow_keys_to_navigate,
                        color: NAVIGATION_MESSAGE_NOTIFICATION
                    });
                }
            },
            div { class: class!(flex_none),
                tabindex: "-1",
                margin_top: "1px",
                button {
                    tabindex: "0",
                    onclick: move |_| { spawn(dictionaryLookup(word_str.clone())); },
                    Icon {
                        height: 16,
                        width: 16,
                        icon: FiSearch,
                    }
                }
            }
            div { class: class!(flex_1),
                tabindex: "-1",
                margin: "2px",
                button { class: class!(inline_block underline),
                    tabindex: "0",
                    onclick: move |_| { spawn(openUrl(url.clone())); },
                    "{word_str}"
                }
                p { class: class!(text_xs),
                    tabindex: "-1",
                    "{word_class}"
                }
            }
            div { class: class!(flex_1),
                tabindex: "-1",
                margin: "2px",
                "{note}"
            }
            div { class: class!(flex_none),
                tabindex: "-1",
                margin: "2px",
                button { class: class!(inline_block),
                    tabindex: "0",
                    onclick: move |event| {
                        event.stop_propagation();
                        word_key.send(WordKey{ id });
                    },
                    Icon {
                        height: 16,
                        width: 16,
                        icon: FiTrash,
                    }
                }
            }
        }
    }
}
