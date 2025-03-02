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

use crate::storage_global::get_storage;
use crate::ui::msg_data_successfully_imported;

#[component]
pub(crate) fn ImportData() -> Element {
    let message_str = use_signal(|| String::new());

    rsx! {
        div { class: class!(text_base),
            margin_top: "5px",
            "Import"
            div {
                margin_top: "5px",
                input {
                    r#type: "file",
                    accept: ".json",
                    onchange: move |event| async move {
                        import(message_str, event).await;
                    }
                }
            }
            div {
                margin_top: "5px",
                "{message_str}"
            }
        }
    }
}

async fn import(
    mut message_str: Signal<String>,
    event: Event<FormData>,
) {
    if let Some(file_engine) = event.files() {
        let files = file_engine.files();
        for file_name in &files {
            if let Some(file) = file_engine.read_file_to_string(file_name).await {
                let result = get_storage().import_data(file).await;
                match result {
                    Ok(_) => message_str.set(msg_data_successfully_imported.to_string()),
                    Err(err) => message_str.set(err.to_string()),
                }
            }
        }
    }
}
