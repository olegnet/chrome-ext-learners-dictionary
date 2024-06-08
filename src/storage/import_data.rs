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

use log::debug;

use crate::model::Data;
use crate::storage::{
    HasId, ObjStoreName, Storage, StorageError, IMPORT_EXPORT_DATA_VERSION, INVALID_VERSION_ERROR,
};

impl Storage {
    pub(crate) async fn import_data(&self, json: String) -> Result<Data, StorageError> {
        let data: Data = serde_json::from_str(json.as_str())?;
        // debug!("import: data: {:?}", &data);

        if data.version != IMPORT_EXPORT_DATA_VERSION {
            return Err(StorageError::ImportError(INVALID_VERSION_ERROR.to_string()));
        }

        self.import(&data.folders).await?;

        self.import(&data.words).await?;

        Ok(data)
    }

    async fn import<T>(&self, data: &Vec<T>) -> Result<(), StorageError>
    where
        T: serde::Serialize + ObjStoreName + HasId<T>,
    {
        let tc = self.get_transaction(T::OBJ_STORE_NAME)?;
        for value in data {
            let js_value = serde_wasm_bindgen::to_value(value)?;
            let result = tc.store.add(&js_value, None).await?;
            debug!("import: result: {:?}", &result);

            let id: u32 = serde_wasm_bindgen::from_value(result.clone())?;
            debug!("import: id: {:?}", &result);

            let word_with_id = value.set_id(Some(id));
            let js_word_with_id = serde_wasm_bindgen::to_value(&word_with_id)?;

            let result = tc.store.put(&js_word_with_id, Some(&result)).await?;
            debug!("import: put: result: {:?}", &result);
        }
        tc.transaction.commit().await?;
        Ok(())
    }
}
