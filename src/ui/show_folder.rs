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
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fi_icons::FiTrash;

use crate::model::{Folder, FolderKey};
use crate::ui::navigation::NavigationState;
use crate::ui::SELECTED_WORD_INDEX;

#[component]
pub(crate) fn ShowFolder(
    index: i32,
    folder: ReadOnlySignal<Folder>,
    selected_folder_str: Signal<String>,
    words_page_offset: Signal<Option<u32>>,
) -> Element {
    let navigation = use_coroutine_handle::<NavigationState>();
    let folder_key = use_coroutine_handle::<FolderKey>();

    let id = folder().id.unwrap();
    let folder_str = folder().folder;
    let folder_note_str = folder().folder_note;

    let background_color = if let 0 = index % 2 { "lists-second-style" } else { "" };

    rsx! {
        div { class: class!(flex flex_wrap items_baseline min_h_12 background_color),
            margin: "1px",
            onclick: move |_| {
                if selected_folder_str() != folder_str {
                    words_page_offset.set(None);
                    *SELECTED_WORD_INDEX.write() = None;
                }
                selected_folder_str.set(folder_str.to_owned());
                navigation.send(NavigationState::Words);
            },
            div { class: class!(flex_auto),
                margin: "2px",
                button { class: class!(underline),
                    "{folder_str}"
                }
            }
            div { class: class!(flex_auto text_xs),
                "{folder_note_str}"
            }
            div { class: class!(flex_none),
                margin: "2px",
                button { class: class!(inline_block),
                    onclick: move |event| {
                        event.stop_propagation();
                        folder_key.send(FolderKey{ id });
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
