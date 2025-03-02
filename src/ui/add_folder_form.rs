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
    let add_folder = use_coroutine(move |mut rx| {
        to_owned![refresh_folders];
        async move {
            while let Some(folder) = rx.next().await {
                let _ = get_storage().add::<Folder>(&folder).await;
                refresh_folders.toggle();
            }
        }
    });

    rsx! {
        form { class: class!(mx_1 my_4),
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
            div { class: class!(grid grid_cols_3 grid_flow_col_dense gap_2 text_base),
                div { class: class!(col_span_2),
                    label {
                        r#for: "folder",
                        input { class: class!(outline min_w_52),
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
                }
                div { class: class!(text_xs text_red_500 col_span_2),
                    "{folder_error_str}"
                }
                div { class: class!(col_span_2),
                    label {
                        r#for: "folder_note",
                        input { class: class!(outline min_w_52),
                            oninput: move |event| folder_note_str.set(event.value()),
                            placeholder: "note",
                            r#type: "text",
                            id: "folder_note",
                            value: "{folder_note_str}"
                        }
                    }
                }
                div { class: class!(row_span_3 self_center my_4),
                    label {
                        title: "Add folder",
                        button { class: class!(btn btn_sm btn_outline),
                            "Add folder"
                        }
                    }
                }
            }
        }
    }
}
