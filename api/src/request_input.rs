use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct CreateWebsteInput{
    pub url : String
}