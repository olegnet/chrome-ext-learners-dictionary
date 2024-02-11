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

use dioxus::prelude::*;

pub(crate) fn page_left(page_length: Signal<u32>, mut offset: Signal<u32>) {
    let new_off = (offset() as i32) - (page_length() as i32);
    if new_off >= 0 {
        offset.set(new_off as u32);
    }
}

pub(crate) fn page_right(count: u32, page_length: Signal<u32>, mut offset: Signal<u32>) {
    let new_off = offset() + page_length();
    if new_off < count {
        offset.set(new_off);
    }
}
