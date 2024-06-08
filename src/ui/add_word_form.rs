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
use crate::model::Word;

use crate::storage_global::get_storage;
use crate::ui::{BASE_URL, CURRENT_TAB_DATA, msg_folder_name_is_empty, msg_word_is_empty, openUrl};

#[component]
pub(crate) fn AddWordForm(
    search_str: ReadOnlySignal<String>,
    selected_folder_str: ReadOnlySignal<String>,
    selected_folder_error_str: Signal<String>,
    word_str: Signal<String>,
    word_error_str: Signal<String>,
    word_class_str: Signal<String>,
    note_str: Signal<String>,
    refresh_words: Signal<u8>,
) -> Element {
    let add_word = use_coroutine(|mut rx| {
        to_owned![refresh_words];
        async move {
            while let Some(word) = rx.next().await {
                let _ = get_storage().add::<Word>(&word).await;
                refresh_words.toggle();
            }
        }
    });

    let open_url = use_coroutine(|mut rx| async move {
        while let Some(url) = rx.next().await {
            openUrl(url).await;
        }
    });

    let current_tab_data = use_memo(move || CURRENT_TAB_DATA());
    let show_url = current_tab_data().url;
    let show_url = match show_url.strip_prefix(BASE_URL) {
        None => show_url,
        Some(str) => str.to_string()
    };

    rsx! {
        div {
            margin_top: "10px",
            a { class: class!(text_xs),
                href: "#",
                margin_top: "5px",
                onclick: move |_| open_url.send(current_tab_data().url),
                "{show_url}"
            }
            p { class: class!(text_sm),
                margin_top: "5px",
                "Folder: {selected_folder_str}"
            }
            p { class: class!(text_xs text_red_500),
                "{selected_folder_error_str}"
            }
            form {
                action: "",
                onsubmit: move |event| {
                    event.stop_propagation();

                    if selected_folder_str().trim().len() == 0 {
                        selected_folder_error_str.set(msg_folder_name_is_empty.to_string());
                    } else if word_str().trim().len() == 0 {
                        word_error_str.set(msg_word_is_empty.to_string());
                    } else {
                        add_word.send(Word::new(
                            &selected_folder_str(),
                            &word_str(),
                            &word_class_str(),
                            &current_tab_data().url,
                            &note_str(),
                        ));
                        word_str.set(String::new());
                        word_class_str.set(String::new());
                        note_str.set(String::new());
                        selected_folder_error_str.set(String::new());
                        word_error_str.set(String::new());
                    }
                },
                label { class: class!(text_base),
                    r#for: "word",
                    input { class: class!(outline),
                        margin_top: "5px",
                        oninput: move |event|  {
                            word_str.set(event.value());
                            if word_str().trim().len() != 0 {
                                word_error_str.set(String::new());
                            }
                        },
                        placeholder: "word or link name",
                        r#type: "text",
                        id: "word",
                        value: "{word_str}"
                    }
                }
                p { class: class!(text_xs text_red_500),
                    "{word_error_str}"
                }
                label { class: class!(text_base),
                    r#for: "word_class",
                    input { class: class!(outline),
                        margin_top: "5px",
                        oninput: move |event| word_class_str.set(event.value()),
                        placeholder: "word class",
                        r#type: "text",
                        id: "word_class",
                        value: "{word_class_str}"
                    }
                }
                label { class: class!(text_base),
                    r#for: "note",
                    input { class: class!(outline),
                        margin_top: "5px",
                        oninput: move |event| note_str.set(event.value()),
                        placeholder: "note",
                        r#type: "text",
                        id: "note",
                        value: "{note_str}"
                    }
                }
                label {
                    title: "Add word",
                    button { class: class!(btn btn_sm btn_outline text_base),
                        margin_left: "5px",
                        "Add word"
                    }
                }
            }
        }
    }
}
