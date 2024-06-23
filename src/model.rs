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

use std::collections::HashMap;

use chrono::Utc;
use lazy_static::lazy_static;
use rexie::Direction;
use rexie::Direction::{Next, Prev};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct Folder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub folder: String,
    pub folder_note: String,
    pub datetime: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct Word {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub folder: String,
    pub word: String,
    pub word_class: String,
    pub url: String,
    pub note: String,
    pub datetime: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct Data {
    pub version: u32,
    pub folders: Vec<Folder>,
    pub words: Vec<Word>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct FoldersAndCount {
    pub folders: Vec<Folder>,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct WordsAndCount {
    pub words: Vec<Word>,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct WordKey {
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct FolderKey {
    pub id: u32,
}

pub const sort_directions: [(&'static str, Direction); 2] =
    [("\u{2191}", Next), ("\u{2193}", Prev)];

lazy_static! {
    pub static ref default_sort_direction: &'static str = sort_directions.get(0).unwrap().0;
    pub static ref sort_direction_map: HashMap<&'static str, Direction> =
        HashMap::from(sort_directions);
}

impl Folder {
    pub fn new(folder: &String, folder_note: &String) -> Folder {
        Folder {
            id: None,
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
            id: None,
            folder: folder.clone(),
            word: word.clone(),
            word_class: word_class.clone(),
            url: url.clone(),
            note: note.clone(),
            datetime: Utc::now().timestamp_millis(),
        }
    }
}
