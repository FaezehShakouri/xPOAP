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

use axum::{extract::State, Extension, Json};
use bonsai_sdk::{alpha::responses::CreateSessRes, alpha_async::get_client_from_parts};
use risc0_ethereum_contracts::i_bonsai_relay::CallbackRequestFilter;

use super::{request_extractor::RequestExtractor, state::ApiState, Error, Result};
use crate::{
    downloader::{
        event_processor::EventProcessor,
        proxy_callback_proof_processor::ProxyCallbackProofRequestProcessor,
    },
    sdk::client::CallbackRequest,
    storage::Storage,
};

/// Publish a CallbackRequest to the Relayer.
///
/// Return status 200 with a body `CreateSessRes`on success.
#[utoipa::path(
    post,
    path = "/v1/callbacks",
    request_body = CallbackRequest,
    responses(
        (status = 200, description = "Callback request sent successfully", body = [CreateSessRes]),
        (status = 400, description = "Bad request error"),
        (status = 500, description = "Internal server error"),
    )
)]
pub(crate) async fn post_callback_request<S: Storage + Sync + Send + Clone>(
    Extension(api_key): Extension<String>,
    State(state): State<ApiState<S>>,
    RequestExtractor(request): RequestExtractor<CallbackRequest>,
) -> Result<Json<CreateSessRes>, Error> {
    let client = get_client_from_parts(state.bonsai_url, api_key, risc0_zkvm::VERSION).await?;
    let proxy =
        ProxyCallbackProofRequestProcessor::new(client, state.storage, Some(state.notifier));
    let session_id = proxy.process_event(request.into()).await?;
    Ok(Json(CreateSessRes {
        uuid: session_id.uuid,
    }))
}

impl From<CallbackRequest> for CallbackRequestFilter {
    fn from(val: CallbackRequest) -> Self {
        CallbackRequestFilter {
            account: val.callback_contract,
            image_id: val.image_id,
            input: val.input.into(),
            callback_contract: val.callback_contract,
            function_selector: val.function_selector,
            gas_limit: val.gas_limit,
        }
    }
}
