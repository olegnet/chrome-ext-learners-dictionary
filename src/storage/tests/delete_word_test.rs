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

    use crate::model::Word;
    use crate::storage::Storage;
    use crate::tests::test_init;

    #[wasm_bindgen_test(async)]
    async fn delete_word_test() {
        test_init("delete_word_test");
        let storage = Storage::open().await.unwrap();

        let mut data: HashMap<u32, Word> = HashMap::with_capacity(10);

        for i in 0..10 {
            let word = format!("word-{}", i);
            let word_class = format!("word-class-{}", i);
            let url = format!("url-{}", i);
            let note = format!("note-{}", i);
            let folder = format!("folder-{}-10", i);
            let id = storage
                .add::<Word>(&Word::new(&folder, &word, &word_class, &url, &note))
                .await
                .unwrap();
            // debug!("add_word: {:?}", &id);

            let word = Word {
                id: Some(id),
                folder,
                word,
                word_class,
                url,
                note,
                datetime: 0,
            };
            data.insert(id, word);
        }

        for (id, word) in data {
            let result = storage.get_word_by_id(id).await.unwrap();
            // debug!("get_by_id: {:?}", &result);
            assert_eq!(word.word, result.word);
            assert_eq!(word.word_class, result.word_class);
            assert_eq!(word.note, result.note);
            assert_ne!(word.datetime, result.datetime);

            storage.delete_by_id::<Word>(word.id.unwrap()).await.unwrap();

            let result = storage.get_word_by_id(word.id.unwrap()).await;
            // debug!("get_by_id: {:?}", &result);
            assert!(matches!(result, Err(_)));
        }
    }
}
