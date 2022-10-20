use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Image
{
    #[serde(rename = "Id")]
    id : String
}