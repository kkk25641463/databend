// Copyright 2021 Datafuse Labs
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

use databend_common_exception::Result;

use crate::servers::flight::v1::exchange::DataExchangeManager;
use crate::servers::flight::v1::packets::QueryEnv;
pub static INIT_QUERY_ENV: &str = "/actions/new_init_query_env";

pub async fn init_query_env(env: QueryEnv) -> Result<()> {
    if let Err(cause) = DataExchangeManager::instance().init_query_env(&env).await {
        DataExchangeManager::instance().on_finished_query(&env.query_id);
        return Err(cause);
    }

    Ok(())
}
