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

use rexie::{Store, Transaction, TransactionMode};

use crate::storage::Storage;
use crate::storage::storage_error::StorageError;

pub(crate) struct TransactionContext {
    pub(crate) transaction: Transaction,
    pub(crate) store: Store,
}

impl Storage {
    pub(crate) fn get_transaction(
        &self,
        store_name: &'static str,
    ) -> Result<TransactionContext, StorageError> {
        let transaction = self
            .rexie
            .transaction(&[store_name], TransactionMode::ReadWrite)?;
        let store = transaction.store(store_name)?;

        Ok(TransactionContext { transaction, store })
    }
}
