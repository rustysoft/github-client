#[derive(Deserialize, Debug)]
pub struct Head {
    #[serde(rename = "ref")]
    pub(in crate) branch: String,
}
