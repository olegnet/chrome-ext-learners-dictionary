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

use crate::model::Folder;
use crate::storage::{OBJ_STORE_FOLDERS, Storage, StorageError};

impl Storage {
    pub(crate) async fn add_folder(
        &self,
        folder: &Folder,
    ) -> Result<(), StorageError> {
        let folder = serde_wasm_bindgen::to_value(&folder)?;

        let tc = self.get_transaction(OBJ_STORE_FOLDERS)?;

        let result = tc.store.add(&folder, None).await?;
        trace!("[wasm] add_folder: result: {:?}", &result);

        tc.transaction.commit().await?;

        Ok(())
    }
}
