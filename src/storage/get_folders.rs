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

use crate::model::{Folder, FoldersAndCount, sort_direction_map};
use crate::storage::{OBJ_STORE_FOLDERS, Storage, StorageError};

impl Storage {
    pub(crate) async fn get_folders(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        direction: String,
    ) -> Result<FoldersAndCount, StorageError> {
        let direction = sort_direction_map
            .get(direction.as_str())
            .unwrap_or(&Prev)
            .to_owned();

        let store = self.get_store(OBJ_STORE_FOLDERS)?;

        let folders: Vec<Folder> = store
            .get_all(None, limit, offset, Some(direction))
            .await?
            .into_iter()
            .map(|value| {
                serde_wasm_bindgen::from_value(value.1).unwrap() // FIXME
            })
            .collect();

        let count = store.count(None).await?;

        Ok(FoldersAndCount { folders, count })
    }
}
