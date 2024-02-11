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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::storage::{DATABASE_NAME, OBJ_STORE_FOLDERS, OBJ_STORE_WORDS};
    use crate::storage::DATABASE_VERSION;
    use crate::storage::Storage;
    use crate::tests::test_init;

    #[wasm_bindgen_test(async)]
    async fn storage_open_test() {
        test_init("storage_open_test");

        let storage = Storage::open().await.unwrap();
        assert_eq!(DATABASE_NAME, &storage.rexie.name());
        assert_eq!(
            <u32 as Into<f64>>::into(DATABASE_VERSION),
            storage.rexie.version()
        );
        assert_eq!("folders", storage.rexie.store_names().get(0).unwrap());
        assert_eq!("words", storage.rexie.store_names().get(1).unwrap());
        assert_eq!(2, storage.rexie.store_names().len());

        let folders_index_names = storage.get_store(OBJ_STORE_FOLDERS)
            .unwrap()
            .index_names();
        assert_eq!(vec!["datetime", "folder"], folders_index_names);

        let words_index_names = storage.get_store(OBJ_STORE_WORDS)
            .unwrap()
            .index_names();
        assert_eq!(vec!["datetime", "folder", "word"], words_index_names);
    }
}
