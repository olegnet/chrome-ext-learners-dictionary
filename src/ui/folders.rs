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

use crate::model::sort_directions;
use crate::storage_global::get_storage;
use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use log::debug;

use crate::ui::add_folder_form::AddFolderForm;
use crate::ui::error_message::ErrorMessage;
use crate::ui::show_folder::ShowFolder;
use crate::ui::utils::{page_left, page_right};

#[component]
pub(crate) fn Folders(
    folder_str: Signal<String>,
    folder_note_str: Signal<String>,
    folder_error_str: Signal<String>,
    selected_folder_str: Signal<String>,
    page_length: Signal<u32>,
    offset: Signal<u32>,
    direction: Signal<String>,
    refresh_folders: Signal<u8>,
    show_add_folder_form: Signal<u8>,
) -> Element {
    debug!("Folders: {} {}", refresh_folders.peek(), show_add_folder_form.peek());

    let folders = use_resource(move || async move {
        let _ = refresh_folders();
        get_storage()
            .get_folders(Some(page_length()), Some(offset()), direction())
            .await
    });

    let (folders_to_show, count) = match &*folders.read_unchecked() {
        None => {
            debug!("None");
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
                for folder in &result.folders {
                    ShowFolder {
                        folder: folder.to_owned(),
                        selected_folder_str: selected_folder_str,
                    }
                }
            },
            result.count,
        ),
    };

    let sort_directions_rendered = sort_directions.iter().map(|(dir, _)| {
        rsx! {
            option { value: "{dir}",
                "{dir}"
            }
        }
    });

    rsx! {
        if show_add_folder_form() != 0u8 {
            AddFolderForm {
                folder_str: folder_str,
                folder_note_str: folder_note_str,
                folder_error_str: folder_error_str,
                refresh_folders: refresh_folders,
            }
        }
        div { class: class!(text_lg),
            margin_top: "5px",
            div { class: class!(hidden),
                "{refresh_folders}"
            }
            div {
                form { action: "",
                    onsubmit: move |event| event.stop_propagation(),
                    select { name: "order",
                        id: "order",
                        onchange: move |event| direction.set(event.value()),
                        {sort_directions_rendered}
                    }
                    // FIXME A <label> isn't associated with a form field.
                    label { class: class!(text_green_500),
                        margin_left: "5px",
                        title: "<<",
                        a { href: "#",
                            onclick: move |_| page_left(page_length, offset),
                            "<<"
                        }
                    }
                    label { class: class!(text_green_500),
                        margin_left: "5px",
                        title: ">>",
                        a { href: "#",
                            onclick: move |_| page_right(count, page_length, offset),
                            ">>"
                        }
                    }
                    label { class: class!(text_red_500),
                        margin_left: "10px",
                        title: "count",
                        "{count}"
                    }
                }
                div {
                    {folders_to_show}
                }
            }
        }
    }
}
