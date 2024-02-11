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

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum StorageError {
    #[error("rexie::Error {0}")]
    RexieError(#[from] rexie::Error),
    #[error("serde_wasm_bindgen::Error {0}")]
    SerdeWasmBindgenError(String),
    #[error("serde_json::Error {0}")]
    SerdeError(String),
    #[error("ImportError {0}")]
    ImportError(String),
}

impl From<serde_wasm_bindgen::Error> for StorageError {
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        print!("serde_wasm_bindgen::Error {}", value);
        StorageError::SerdeWasmBindgenError(value.to_string())
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(value: serde_json::Error) -> Self {
        print!("serde_json::Error {}", value);
        StorageError::SerdeError(value.to_string())
    }
}
