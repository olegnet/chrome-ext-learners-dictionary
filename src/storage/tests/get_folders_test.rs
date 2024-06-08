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
    use std::collections::HashMap;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::storage::Storage;
    use crate::model::Folder;
    use crate::tests::test_init;

    #[ignore = "should run separately"]
    #[wasm_bindgen_test(async)]
    async fn get_folders_test() {
        test_init("get_folders_test");
        let storage = Storage::open().await.unwrap();

        let mut data: Vec<Folder> = vec![];
        let length = 10;

        for i in 0..length {
            let folder = format!("folder-{}-4", i);
            let folder_note = format!("note-{}", i);
            let folder2 = Folder::new(&folder, &folder_note);
            storage.add::<Folder>(&folder2).await.unwrap();
            data.push(folder2);
        }

        let err = storage
            .add::<Folder>(&Folder::new(&"folder-0-4".to_string(), &"note-0".to_string()))
            .await;
        // debug!("err: {:?}", err);
        assert!(matches!(err, Err(_)));

        let result = storage.get_folders(None, None, String::new()).await.unwrap();
        // debug!("result.len(): {:?}", &result.folders.len());
        assert_eq!(&(data.len()), &result.folders.len());

        let mut result_map: HashMap<String, Folder> = HashMap::with_capacity(length as usize);
        result.folders.iter().for_each(|x| {
            result_map.insert(x.folder.clone(), x.to_owned());
        });

        for row in &data {
            let result = result_map.get(&row.folder).unwrap();
            // debug!("result: {:?}", &result);
            assert_ne!(row.id, result.id);
            assert_eq!(row.folder, result.folder);
            assert_eq!(row.folder_note, result.folder_note);
            assert_eq!(row.datetime, result.datetime);
        }
    }
}
