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
    use crate::model::WordKey;

    use crate::storage::Storage;
    use crate::model::Word;
    use crate::tests::test_init;

    #[wasm_bindgen_test(async)]
    async fn get_words_test() {
        test_init("get_words_test");
        let storage = Storage::open().await.unwrap();

        let mut data: Vec<Word> = vec![];
        let length = 10;

        let folder = "folder-0-2".to_string();

        for i in 0..length {
            let word = format!("word-{}-2", i);
            let word_class = format!("word-class-{}", i);
            let url = format!("url-{}", i);
            let note = format!("note-{}", i);
            let _id = storage
                .add_word(&Word::new(&folder, &word, &word_class, &url, &note))
                .await
                .unwrap();
            // debug!("get_words_test: {:?}", &id);

            data.push(Word {
                folder: folder.clone(),
                word,
                word_class,
                url,
                note,
                datetime: 0,
            });
        }

        let result = storage
            .get_words(folder.clone(), Some(length), Some(0), "ascending".to_string())
            .await
            .unwrap();
        // debug!("result.len(): {:?}", &result.len());
        assert_eq!(&data.len(), &result.words.len());

        let mut result_map: HashMap<WordKey, Word> = HashMap::with_capacity(length as usize);
        result.words.iter().for_each(|x| {
            let key = (x.folder.clone(), x.word.clone());
            result_map.insert(key, x.to_owned());
        });

        for row in &data {
            let key = (row.folder.clone(), row.word.clone());
            let result = result_map.get(&key).unwrap();
            // debug!("result: {:?}", &result);
            assert_eq!(row.folder, result.folder);
            assert_eq!(row.word, result.word);
            assert_eq!(row.word_class, result.word_class);
            assert_eq!(row.note, result.note);
            assert_ne!(row.datetime, result.datetime);
        }

        let count = result.count;
        // debug!("count: {}", count);
        assert_eq!(length, count);
    }
}
