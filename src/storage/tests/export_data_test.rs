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

    use crate::model::{Data, Folder, Word};
    use crate::storage::{IMPORT_EXPORT_DATA_VERSION, Storage};
    use crate::tests::test_init;

    #[wasm_bindgen_test(async)]
    async fn export_data_test() {
        test_init("export_data_test");
        let storage = Storage::open().await.unwrap();

        let mut folders: Vec<Folder> = vec![];
        let mut words: Vec<Word> = vec![];
        let folders_len = 10;
        let words_len = 10;

        for i in 0..folders_len {
            let folder_name = format!("folder-{}-6", i);
            let folder_note = format!("note-{}", i);
            let folder = Folder::new(&folder_name, &folder_note);
            storage.add_folder(&folder).await.unwrap();
            folders.push(folder);

            for j in 0..words_len {
                let word = format!("word-{}-6", j);
                let word_class = format!("word-class-{}", j);
                let url = format!("url-{}", j);
                let note = format!("note-{}", j);
                let id = storage
                    .add_word(&Word::new(&folder_name, &word, &word_class, &url, &note))
                    .await
                    .unwrap();

                words.push(Word {
                    id: Some(id),
                    folder: folder_name.clone(),
                    word,
                    word_class,
                    url,
                    note,
                    datetime: 0,
                });
            }
        }

        let json_str = storage.export_data().await.unwrap();
        // debug!("result: {:?}", &json_str);

        let data: Data = serde_json::from_str(json_str.as_str()).unwrap();
        assert_eq!(IMPORT_EXPORT_DATA_VERSION, data.version);
        // assert_eq!(folders_len, data.folders.len());
        // assert_eq!(folders_len * words_len, data.words.len());

        let mut result_map: HashMap<String, Folder> = HashMap::new();
        data.folders.iter().for_each(|x| {
            result_map.insert(x.folder.clone(), x.to_owned());
        });
        for row in &folders {
            let result = result_map.get(&row.folder).unwrap();
            // debug!("result: {:?}", &result);
            assert_eq!(row, result);
        }

        let mut result_map: HashMap<(String, String), Word> = HashMap::new();
        data.words.iter().for_each(|x| {
            let key = (x.folder.clone(), x.word.clone());
            result_map.insert(key, x.to_owned());
        });
        for row in &words {
            let key = (row.folder.clone(), row.word.clone());
            let result = result_map.get(&key).unwrap();
            // debug!("result: {:?}", &result);
            assert_eq!(row.folder, result.folder);
            assert_eq!(row.word, result.word);
            assert_eq!(row.word_class, result.word_class);
            assert_eq!(row.note, result.note);
            assert_ne!(row.datetime, result.datetime);
        }
    }
}
