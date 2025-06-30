// sort of similar to zod sort of writing out schema

// shape out out data -->

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String,
}
