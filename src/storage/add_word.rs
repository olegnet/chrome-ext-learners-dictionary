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

use log::trace;

use crate::model::Word;
use crate::storage::{OBJ_STORE_WORDS, Storage, StorageError};

impl Storage {
    pub(crate) async fn add_word(&self, word: &Word) -> Result<u32, StorageError> {
        let js_word = serde_wasm_bindgen::to_value(&word)?;

        let tc = self.get_transaction(OBJ_STORE_WORDS)?;

        let result = tc.store.add(&js_word, None).await?;
        trace!("[wasm] add_word: add: result: {:?}", &result);

        let id: u32 = serde_wasm_bindgen::from_value(result.clone())?;
        let mut word_with_id = word.clone();
        word_with_id.id = Some(id);
        let js_word_with_id = serde_wasm_bindgen::to_value(&word_with_id)?;

        let result = tc.store.put(&js_word_with_id, Some(&result)).await?;
        trace!("[wasm] add_word: put: result: {:?}", &result);

        tc.transaction.commit().await?;

        Ok(id)
    }
}
