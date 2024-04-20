// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use tokio::sync::Notify;

use crate::storage::Storage;

#[derive(Clone)]
pub(crate) struct ApiState<S>
where
    S: Storage + Sync + Send + Clone,
{
    pub(crate) bonsai_url: String,
    pub(crate) storage: S,
    pub(crate) notifier: Arc<Notify>,
}
