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

use rexie::Direction::Prev;
use rexie::KeyRange;
use wasm_bindgen::JsValue;

use crate::model::{sort_direction_map, Word, WordsAndCount};
use crate::storage::{INDEX_FOLDER, Storage, StorageError};

impl Storage {
    pub(crate) async fn get_words(
        &self,
        folder: String,
        limit: Option<u32>,
        offset: Option<u32>,
        direction: String,
    ) -> Result<WordsAndCount, StorageError> {
        let key_range = Self::key_range_only(folder)?;

        let direction = sort_direction_map
            .get(direction.as_str())
            .unwrap_or(&Prev)
            .to_owned();

        let store_index = self.store_index(&INDEX_FOLDER)?;

        let words: Vec<Word> = store_index
            .get_all(Some(&key_range), limit, offset, Some(direction))
            .await?
            .into_iter()
            .map(|value| {
                serde_wasm_bindgen::from_value(value.1).unwrap() // FIXME
            })
            .collect();

        let count = store_index.count(Some(&key_range)).await?;

        Ok(WordsAndCount { words, count })
    }

    fn key_range_only(string: String) -> Result<KeyRange, StorageError> {
        Ok(KeyRange::only(&JsValue::from(&string))?)
    }
}
