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

    use crate::model::Folder;
    use crate::storage::Storage;
    use crate::model::Word;
    use crate::tests::test_init;

    #[wasm_bindgen_test(async)]
    async fn add_folders_and_words_test() {
        test_init("add_folders_and_words_test");
        let storage = Storage::open().await.unwrap();

        let folder_one_name = "folder-1-5".to_string();
        let folder_two_name = "folder-2-5".to_string();

        let err = storage
            .add_folder(&Folder::new(&folder_one_name, &"folder-note-1".to_string()))
            .await;
        // debug!("err: {:?}", err);
        assert_eq!(err, Ok(()));

        let err = storage
            .add_folder(&Folder::new(&folder_two_name, &"folder-note-2".to_string()))
            .await;
        // debug!("err: {:?}", err);
        assert_eq!(err, Ok(()));

        let word_one_name = "word-1-5".to_string();
        let word_two_name = "word-2-5".to_string();
        let word_three_name = "word-3-5".to_string();
        let word_four_name = "word-4-5".to_string();

        let word_one = Word::new(
            &folder_one_name,
            &word_one_name,
            &"noun".to_string(),
            &"url-1".to_string(),
            &"note-1".to_string(),
        );
        let word_two = Word::new(
            &folder_two_name,
            &word_two_name,
            &"verb".to_string(),
            &"url-2".to_string(),
            &"note-2".to_string(),
        );

        let word_three = Word::new(
            &folder_one_name,
            &word_three_name,
            &"noun".to_string(),
            &"url-3".to_string(),
            &"note-3".to_string(),
        );
        let word_four = Word::new(
            &folder_two_name,
            &word_four_name,
            &"verb".to_string(),
            &"url-4".to_string(),
            &"note-4".to_string(),
        );

        let _word_one_id = storage.add_word(&word_one).await.unwrap();
        // debug!("word_one: {:?}", &word_one_id);

        let _word_two_id = storage.add_word(&word_two).await.unwrap();
        // debug!("word_two: {:?}", &word_two_id);

        let _word_three_id = storage.add_word(&word_three).await.unwrap();
        // debug!("word_three: {:?}", &word_three_id);

        let _word_four_id = storage.add_word(&word_four).await.unwrap();
        // debug!("word_four: {:?}", &word_four_id);

        let result = storage
            .get_words(folder_one_name, None, None, "ascending".to_string())
            .await
            .unwrap();
        // debug!("result.len(): {:?}", &result.len());
        // for row in &result {
        //     debug!("result: {:?}", row);
        // }
        assert_eq!(2, result.words.len());
    }
}
