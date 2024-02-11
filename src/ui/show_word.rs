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
use dioxus_free_icons::icons::fi_icons::FiSearch;

use crate::model::Word;
use crate::ui::{dictionaryLookup, openUrl};

#[component]
pub(crate) fn ShowWord(word: ReadOnlySignal<Word>) -> Element {
    let word_str = word().word;
    let word_class = word().word_class;
    let note = word().note;
    let url = word().url;

    rsx! {
        div { //class: class!(grid),
            div { class: class!(flex items_baseline),
                a { class: class!(inline_block underline),
                    href: "#",
                    onclick: move |_| { spawn(openUrl(url.clone())); },
                    "{word_str}"
                }
                a { class: class!(inline_block),
                    margin_left: "10px",
                    href: "#",
                    onclick: move |_| { spawn(dictionaryLookup(word_str.clone())); },
                    Icon {
                        fill: "black",
                        height: 15,
                        width: 15,
                        icon: FiSearch,
                    }
                }
                span { class: class!(text_xs inline_block),
                    margin_left: "5px",
                    "{word_class}"
                }
                span { class: class!(inline_block),
                    margin_left: "5px",
                    "{note}"
                }
            }
        }
    }
}
