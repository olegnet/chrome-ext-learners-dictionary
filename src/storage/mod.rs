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

use std::fmt::Debug;

use rexie::Rexie;

use crate::model::{Folder, Word};
use crate::storage::storage_error::StorageError;

mod add_folder;
mod add_word;
mod delete_word;
mod export_data;
mod get_folders;
mod get_store;
mod get_transaction;
mod get_word_by_id;
mod get_words;
mod import_data;
mod open;
mod storage_error;
mod store_index;
mod tests;

const DATABASE_NAME: &str = "dictionary";
const DATABASE_VERSION: u32 = 1;

const OBJ_STORE_FOLDERS: &str = "folders";
const OBJ_STORE_WORDS: &str = "words";

const INDEX_FOLDER: &str = "folder";

const IMPORT_EXPORT_DATA_VERSION: u32 = 1;

const INVALID_VERSION_ERROR: &str = "Invalid version";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Storage {
    rexie: Rexie,
}

trait ObjStoreName {
    const OBJ_STORE_NAME: &'static str;
}

impl ObjStoreName for Folder {
    const OBJ_STORE_NAME: &'static str = OBJ_STORE_FOLDERS;
}

impl ObjStoreName for Word {
    const OBJ_STORE_NAME: &'static str = OBJ_STORE_WORDS;
}

trait HasId<T> {
    const USE_ID: bool;

    fn set_id(&self, id: Option<u32>) -> T;
}

impl HasId<Folder> for Folder {
    const USE_ID: bool = false;

    fn set_id(&self, _id: Option<u32>) -> Folder {
        self.clone()
    }
}

impl HasId<Word> for Word {
    const USE_ID: bool = true;

    fn set_id(&self, id: Option<u32>) -> Word {
        let mut word = self.clone();
        word.id = id;
        word
    }
}
