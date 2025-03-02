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

use std::cell::RefCell;
use std::rc::Rc;

use crate::storage::Storage;

thread_local! {
    static STORAGE: RefCell<Option<Rc<Storage>>> = const { RefCell::new(None) }
}

pub async fn init_storage() {
    let result = Storage::open().await;
    match result {
        Ok(storage) => STORAGE.with(|cell| {
            cell.replace(Some(Rc::new(storage)));
        }),
        Err(err) => panic!("init_storage(): {:?}", &err),
    }
}

pub fn get_storage() -> Rc<Storage> {
    STORAGE.with(|cell| {
        cell.borrow()
            .as_ref()
            .expect("init_storage() must be called before get_storage()")
            .to_owned()
    })
}
