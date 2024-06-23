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
use log::debug;

use crate::storage_global::get_storage;
use crate::ui::add_word_form::AddWordForm;
use crate::ui::error_message::ErrorMessage;
use crate::ui::pager::Pager;
use crate::ui::search_form::SearchForm;
use crate::ui::show_word::ShowWord;

#[component]
pub(crate) fn Words(
    selected_folder_str: Signal<String>,
    selected_folder_error_str: Signal<String>,
    word_str: Signal<String>,
    word_error_str: Signal<String>,
    word_class_str: Signal<String>,
    note_str: Signal<String>,
    page_length: Signal<Option<u32>>,
    offset: Signal<Option<u32>>,
    direction: Signal<String>,
    refresh_words: Signal<u8>,
    show_add_word_form: Signal<u8>,
) -> Element {
    // debug!("Words: {}", refresh_words.peek());

    let search_str = use_signal(|| String::new());

    let words = use_resource(move || async move {
        let _ = refresh_words();
        get_storage()
            .get_words(selected_folder_str(), page_length(), offset(), direction())
            .await
    });

    let (words_to_show, count) = match &*words.read_unchecked() {
        None => {
            // debug!("None");
            (None, 0u32)
        }
        Some(Err(err)) => {
            debug!("Err: {}", err);
            (
                rsx! {
                    ErrorMessage {
                        message: err.to_string()
                    }
                },
                0u32,
            )
        }
        Some(Ok(result)) => (
            rsx! {
                for (index, word) in result.words.iter().enumerate() {
                    ShowWord {
                        word: word.to_owned(),
                        background_color: match index % 2 { 0 => "lists-second-colors", _ => "" },
                    }
                }
            },
            result.count,
        ),
    };

    rsx! {
        if show_add_word_form() != 0u8 {
            SearchForm {
                search_str: search_str,
            }

            AddWordForm {
                search_str: search_str,
                selected_folder_str: selected_folder_str,
                selected_folder_error_str: selected_folder_error_str,
                word_str: word_str,
                word_error_str: word_error_str,
                word_class_str: word_class_str,
                note_str: note_str,
                refresh_words: refresh_words,
            }
        } else {
            div { class: class!(text_base),
                margin_top: "5px",
                "Folder: "
                span { class: class!(italic),
                    "{selected_folder_str}"
                }
            }
        }

        div { class: class!(text_lg),
            margin_top: "5px",
            div { class: class!(hidden),
                "{refresh_words}"
            }
            Pager {
                page_length: page_length,
                offset: offset,
                direction: direction,
                count: count,
            }
            div {
                {words_to_show}
            }
        }
    }
}
