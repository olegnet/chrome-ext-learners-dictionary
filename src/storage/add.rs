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

use rexie::Store;

use crate::storage::{HasId, ObjStoreName, Storage, StorageError};

impl Storage {
    pub(crate) async fn add<T>(&self, value: &T) -> Result<u32, StorageError>
    where
        T: serde::Serialize + ObjStoreName + HasId<T>,
    {
        let tc = self.get_transaction(T::OBJ_STORE_NAME)?;

        let id = self.add_value(&tc.store, value).await?;

        tc.transaction.commit().await?;

        Ok(id)
    }

    pub(crate) async fn import<T>(&self, data: &Vec<T>) -> Result<(), StorageError>
    where
        T: serde::Serialize + ObjStoreName + HasId<T>,
    {
        let tc = self.get_transaction(T::OBJ_STORE_NAME)?;

        for value in data {
            self.add_value(&tc.store, value).await?;
        }

        tc.transaction.commit().await?;

        Ok(())
    }

    async fn add_value<T>(&self, store: &Store, value: &T) -> Result<u32, StorageError>
    where
        T: serde::Serialize + HasId<T>,
    {
        let js_value = serde_wasm_bindgen::to_value(value)?;
        let result = store.add(&js_value, None).await?;
        // debug!("add_vec: result: {:?}", &result);

        let id: u32 = serde_wasm_bindgen::from_value(result.clone())?;
        // debug!("add_vec: id: {:?}", &result);

        let word_with_id = value.set_id(Some(id));
        let js_word_with_id = serde_wasm_bindgen::to_value(&word_with_id)?;

        let _result = store.put(&js_word_with_id, Some(&result)).await?;
        // debug!("add_vec: put: result: {:?}", &result);

        Ok(id)
    }
}
