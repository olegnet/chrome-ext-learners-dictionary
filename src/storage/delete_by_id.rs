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

use crate::storage::{ObjStoreName, Storage};
use crate::storage::storage_error::StorageError;

impl Storage {
    pub(crate) async fn delete_by_id<T>(&self, id: u32) -> Result<(), StorageError>
        where T: ObjStoreName
    {
        let key = serde_wasm_bindgen::to_value(&id)?;

        let tc = self.get_transaction(T::OBJ_STORE_NAME)?;

        let result = tc.store.delete(&key).await?;

        tc.transaction.commit().await?;

        Ok(result)
    }
}
