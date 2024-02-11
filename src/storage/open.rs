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

use rexie::{Index, ObjectStore, Rexie};

use crate::storage::{
    DATABASE_NAME, DATABASE_VERSION, OBJ_STORE_FOLDERS, OBJ_STORE_WORDS, Storage,
};
use crate::storage::storage_error::StorageError;

impl Storage {
    pub(crate) async fn open() -> Result<Storage, StorageError> {
        let rexie = Rexie::builder(DATABASE_NAME)
            .version(DATABASE_VERSION)
            .add_object_store(
                ObjectStore::new(OBJ_STORE_FOLDERS)
                    .key_path("folder")
                    .add_index(Index::new_array("folder", ["folder", "folder_note"]))
                    .add_index(Index::new("datetime", "datetime")),
            )
            .add_object_store(
                ObjectStore::new(OBJ_STORE_WORDS)
                    .key_path_array(["folder", "word"])
                    .add_index(Index::new("folder", "folder"))
                    .add_index(Index::new_array(
                        "word",
                        ["folder", "word", "word_class", "note"],
                    ))
                    .add_index(Index::new("datetime", "datetime")),
            )
            .build()
            .await?;

        Ok(Storage { rexie })
    }

    #[allow(dead_code)]
    pub(super) fn close_db(self) {
        self.rexie.close();
    }

    #[allow(dead_code)]
    pub(super) async fn delete_db() -> Result<(), rexie::Error> {
        Rexie::delete(DATABASE_NAME).await
    }
}
