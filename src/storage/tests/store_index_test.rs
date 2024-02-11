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
    use rexie::{Direction, KeyRange};
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::storage::Storage;
    use crate::model::Word;
    use crate::tests::test_init;

    #[wasm_bindgen_test(async)]
    async fn store_index_test() {
        test_init("store_index_test");
        let storage = Storage::open().await.unwrap();

        for i in 0..10 {
            let folder = format!("folder-{}-3", i);
            let word = format!("word-{}-3", i);
            let word_class = format!("word-class-{}", i);
            let url = format!("url-{}", i);
            let note = format!("note-{}", i);
            storage
                .add_word(&Word::new(&word, &word_class, &url, &note, &folder))
                .await
                .unwrap();
        }

        let store_index_url = storage.store_index(&"word").unwrap();
        let js_key_lb = JsValue::from("word-0-3");
        let js_key_ub = JsValue::from("word-9-3");
        let bound_key = KeyRange::bound(&js_key_lb, &js_key_ub, true, true).unwrap();
        let _count = store_index_url.count(None).await.unwrap();
        // debug!("count: {}", count);
        let _js_value = store_index_url.get(&js_key_lb).await.unwrap();
        // debug!("js_value: {:?}", js_value);

        let _words: Vec<Word> = store_index_url
            .get_all(Some(&bound_key), Some(100), Some(0), Some(Direction::Next))
            .await
            .unwrap()
            .into_iter()
            .map(|pair| serde_wasm_bindgen::from_value(pair.1).unwrap())
            .collect();
        // debug!("words: {:?}", words);
    }
}
