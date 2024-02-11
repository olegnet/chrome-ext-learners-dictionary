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

use crate::model::{Data, Folder, Word};
use crate::storage::{IMPORT_EXPORT_DATA_VERSION, ObjStoreName, Storage, StorageError};

impl Storage {
    pub(crate) async fn export_data(&self) -> Result<String, StorageError> {
        let folders: Vec<Folder> = self.export_store().await?;
        let words: Vec<Word> = self.export_store().await?;
        let data = Data {
            version: IMPORT_EXPORT_DATA_VERSION,
            folders,
            words,
        };
        Ok(serde_json::to_string(&data)?)
    }

    async fn export_store<T>(&self) -> Result<Vec<T>, StorageError>
    where
        T: serde::de::DeserializeOwned + ObjStoreName,
    {
        let result: Vec<T> = self
            .get_store(T::OBJ_STORE_NAME)?
            .get_all(None, None, None, None)
            .await?
            .into_iter()
            .map(|value| {
                serde_wasm_bindgen::from_value(value.1).unwrap() // FIXME
            })
            .collect();
        Ok(result)
    }
}
