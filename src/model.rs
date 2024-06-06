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

#![allow(non_upper_case_globals)]

use chrono::Utc;
use lazy_static::lazy_static;
use rexie::Direction;
use rexie::Direction::{Next, Prev};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)] // FIXME
pub struct Folder {
    pub folder: String,
    pub folder_note: String,
    pub datetime: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)] // FIXME
pub struct Word {
    pub folder: String,
    pub word: String,
    pub word_class: String,
    pub url: String,
    pub note: String,
    pub datetime: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)] // FIXME
pub struct Data {
    pub version: u32,
    pub folders: Vec<Folder>,
    pub words: Vec<Word>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)] // FIXME
pub struct FoldersAndCount {
    pub folders: Vec<Folder>,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)] // FIXME
pub struct WordsAndCount {
    pub words: Vec<Word>,
    pub count: u32,
}

pub type WordKey = u32;

pub const sort_directions: [(&'static str, Direction); 2] = [
    ("ascending", Next),
    ("descending", Prev),
];

lazy_static! {
    pub static ref default_sort_direction: &'static str = sort_directions.get(0).unwrap().0;
    pub static ref sort_direction_map: HashMap<&'static str, Direction> =
        HashMap::from(sort_directions);
}

impl Folder {
    pub fn new(folder: &String, folder_note: &String) -> Folder {
        Folder {
            folder: folder.clone(),
            folder_note: folder_note.clone(),
            datetime: Utc::now().timestamp_millis(),
        }
    }
}

impl Word {
    pub fn new(
        folder: &String,
        word: &String,
        word_class: &String,
        url: &String,
        note: &String,
    ) -> Word {
        Word {
            folder: folder.clone(),
            word: word.clone(),
            word_class: word_class.clone(),
            url: url.clone(),
            note: note.clone(),
            datetime: Utc::now().timestamp_millis(),
        }
    }
}
