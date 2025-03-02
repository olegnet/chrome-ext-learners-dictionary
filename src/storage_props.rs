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

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, window};

#[allow(dead_code)] // FIXME
pub async fn log_storage_properties() -> Result<JsValue, JsValue> {
    let promise = window()
        .expect("No window!")
        .navigator()
        .storage()
        .estimate()?;
    let result = JsFuture::from(promise).await;
    match &result {
        Ok(value) => console::log_2(&"navigator.storage.estimate\n".into(), &value),
        Err(err) => console::warn_1(&err),
    }
    result
}

#[cfg(test)]
mod tests {
    use log::debug;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::tests::test_init;

    use super::*;

    #[wasm_bindgen_test(async)]
    async fn log_storage_properties_test() {
        test_init("log_storage_properties_test");

        let result = log_storage_properties().await.unwrap();
        debug!("value: {:?}", result);
    }
}
