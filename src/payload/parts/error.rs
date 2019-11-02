#[derive(Deserialize, Debug)]
pub struct Error {
    resource: String,
    code: String,
    pub(in crate) message: String,
}
