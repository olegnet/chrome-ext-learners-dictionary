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

use crate::model::Data;
use crate::storage::{
    IMPORT_EXPORT_DATA_VERSION, INVALID_VERSION_ERROR, Storage, StorageError,
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
}
