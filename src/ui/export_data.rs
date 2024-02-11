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
use gloo_utils::format::JsValueSerdeExt;
use log::debug;
use wasm_bindgen::JsValue;
use web_sys::{console, File, FilePropertyBag, Url};

use crate::storage_global::get_storage;
use crate::ui::error_message::ErrorMessage;
use crate::ui::navigation::Navigation;
use crate::ui::{startDownload, EXPORT_FILE_NAME, EXPORT_FILE_TYPE, js_value_to_string};

#[component]
pub(crate) fn ExportData() -> Element {
    let export_data = use_resource(move || async move {
        get_storage().export_data().await
    });

    match &*export_data.read_unchecked() {
        None => None,   // FIXME add some placeholder message
        Some(Ok(data)) => {
            rsx! {
                StartDownload {
                    data
                }
            }
        }
        Some(Err(err)) => {
            debug!("Err: {}", err);
            rsx! {
                ErrorMessage {
                    message: err.to_string()
                }
            }
        }
    }
}

#[component]
pub(crate) fn StartDownload(data: String) -> Element {
    let navigation = use_coroutine_handle::<Navigation>();

    let result = open_download_window(&data);
    match result {
        Ok(_) => {
            navigation.send(Navigation::Settings);
            None
        }
        Err(err) => {
            console::warn_1(&err);
            rsx! {
                ErrorMessage {
                    message: js_value_to_string(&err)
                }
            }
        }
    }
}

fn open_download_window(data: &str) -> Result<(), JsValue> {
    let js_value = JsValue::from_serde(&vec![data])
        .map_err(|err| err.to_string())?;   // FIXME to_string() ?

    let file = File::new_with_str_sequence_and_options(
        &js_value,
        EXPORT_FILE_NAME, // this name isn't used, look below
        &FilePropertyBag::new().type_(EXPORT_FILE_TYPE),
    )?;

    let url = Url::create_object_url_with_blob(&file)?;

    startDownload(url, EXPORT_FILE_NAME.to_string());

    // FIXME Url::revoke_object_url(&url);

    Ok(())
}
