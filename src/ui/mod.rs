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
#![allow(non_upper_case_globals)]

use dioxus::prelude::*;
use log::debug;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::ui::navigation::Navigation;

mod add_folder_form;
mod add_word_form;
mod error_message;
mod export_data;
mod folders;
mod import_data;
mod navigation;
mod search_form;
mod settings;
mod show_folder;
mod show_word;
mod words;
mod pager;
mod page_length;
mod show_copyright;

#[derive(Default, Clone, PartialEq)]
struct CurrentTabData {
    url: String,
    word: String,
    word_class: String,
    phonetics: String,
}

const msg_folder_name_is_empty: &str = "Folder name is empty";
const msg_word_is_empty: &str = "Word is empty";
const msg_select_folder_first: &str = "Please select a folder first";
const msg_data_successfully_imported: &str = "Data successfully imported";

const BASE_URL: &str = "https://www.oxfordlearnersdictionaries.com";

const EXPORT_FILE_NAME: &str = "export.json";
const EXPORT_FILE_TYPE: &str = "application/json";

static CURRENT_TAB_DATA: GlobalSignal<CurrentTabData> = Signal::global(|| CurrentTabData::default());

#[wasm_bindgen]
pub fn on_tab_loaded(url: String, word: String, word_class: String, title: String, phonetics: String) {
    let mut word = word;
    if word.len() == 0 {
        match title.split_once(" - ") {
            Some((str, _)) => word = str.to_string(),
            None => match title.split_once(" | ") {
                Some((str, _)) => word = str.to_string(),
                None => {}
            }
        }
    }

    // debug!("tab is loaded: word={} class={} phonetics={} url={}", word, word_class, phonetics, url);

    match Runtime::current() {
        None => debug!("Runtime::current() is None"),
        Some(_) => ScopeId::ROOT.in_runtime(|| {
            CURRENT_TAB_DATA.with_mut(move |v|
                *v = CurrentTabData { url, word, word_class, phonetics }
            );
        })
    }
}

#[wasm_bindgen(module = "/helper.js")]
extern "C" {
    pub async fn dictionaryLookup(searchText: String);
    pub async fn openUrl(url: String);
    pub async fn updateCurrentTabData();
    pub fn startDownload(url: String, filename: String);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = String)]
    pub fn js_value_to_string(value: &JsValue) -> String;
}

#[component]
pub fn App() -> Element {
    rsx! {
        Navigation {}
    }
}
