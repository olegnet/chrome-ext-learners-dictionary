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

use crate::model::Folder;
use crate::ui::navigation::Navigation;

#[component]
pub(crate) fn ShowFolder(
    folder: ReadOnlySignal<Folder>,
    selected_folder_str: Signal<String>,
) -> Element {
    let navigation = use_coroutine_handle::<Navigation>();

    let folder_str = folder().folder;
    let folder_note_str = folder().folder_note;

    rsx! {
        div { class: class!(flex items_baseline),
            span { class: class!(inline_block),
                a { class: class!(underline),
                    href: "#",
                    onclick: move |_| {
                        selected_folder_str.set(folder_str.to_owned());
                        navigation.send(Navigation::Words);
                    },
                    "{folder_str}"
                }
            }
            span { class: class!(inline_block text_sm),
                margin_left: "5px",
                "{folder_note_str}"
            }
        }
    }
}
