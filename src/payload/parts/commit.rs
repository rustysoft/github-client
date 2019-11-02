#[derive(Deserialize, Debug)]
pub struct Commit {
    pub(in crate) message: String,
}
