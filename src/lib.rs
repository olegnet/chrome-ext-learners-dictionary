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

use cfg_if::cfg_if;
// use dioxus::logger::tracing::Level;
use dioxus::prelude::*;
use log::debug;
use wasm_bindgen::prelude::*;

use crate::storage_global::init_storage;
use crate::ui::App;

mod model;
mod storage;
mod storage_global;
mod storage_props;
mod ui;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            // console_log::init_with_level(log::Level::Trace).expect("Error initializing logger");
        }
    } else {
        fn init_log() {
            // web_sys::console::log_1(&"Logger is switched off".into());
        }
    }
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    init_log();
    debug!("start(): Inited.");

    init_storage().await;

    // log_storage_properties().await?;

    launch(App);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::*;

    pub fn test_init(test_name: &str) {
        wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

        static INIT: Once = Once::new();
        INIT.call_once(|| {
            console_error_panic_hook::set_once();

            console_log::init_with_level(log::Level::Trace).expect("Error initializing logger");
        });

        debug!("--> test started: {}", test_name);
    }
}
