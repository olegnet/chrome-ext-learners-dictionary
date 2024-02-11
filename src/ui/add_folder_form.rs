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

use crate::model::Folder;
use crate::storage_global::get_storage;
use crate::ui::msg_folder_name_is_empty;

#[component]
pub(crate) fn AddFolderForm(
    folder_str: Signal<String>,
    folder_note_str: Signal<String>,
    folder_error_str: Signal<String>,
    refresh_folders: Signal<u8>,
) -> Element {
    let add_folder = use_coroutine(|mut rx| {
        to_owned![refresh_folders];
        async move {
            while let Some(folder) = rx.next().await {
                let _ = get_storage().add_folder(&folder).await;
                refresh_folders.toggle();
            }
        }
    });

    rsx! {
        div { class: class!(grid grid_cols_1 text_base),
            margin_left: "5px",
            form {
                margin_top: "15px",
                action: "",
                onsubmit: move |event| {
                    event.stop_propagation();

                    if folder_str().trim().len() == 0 {
                        folder_error_str.set(msg_folder_name_is_empty.to_string());
                    } else {
                        add_folder.send(Folder::new(
                            &folder_str(),
                            &folder_note_str(),
                        ));
                        folder_str.set(String::new());
                        folder_note_str.set(String::new());
                        folder_error_str.set(String::new());
                    }
                },
                label {
                    r#for: "folder",
                    input { class: class!(outline),
                        oninput: move |event| { 
                            folder_str.set(event.value());
                            if folder_str().trim().len() != 0 {
                                folder_error_str.set(String::new());
                            }
                        },
                        placeholder: "new folder name",
                        r#type: "text",
                        id: "folder",
                        value: "{folder_str}"
                    }
                }
                p { class: class!(text_xs text_red_500),
                    "{folder_error_str}"
                }
                label {
                    r#for: "folder_note",
                    input { class: class!(outline),
                        margin_top: "5px",
                        oninput: move |event| folder_note_str.set(event.value()),
                        placeholder: "note",
                        r#type: "text",
                        id: "folder_note",
                        value: "{folder_note_str}"
                    }
                }
                label {
                    title: "Add folder",
                    button { class: class!(btn btn_sm btn_outline),
                        margin_top: "5px",
                        margin_left: "5px",
                        "Add folder"
                    }
                }
            }
        }
    }
}
