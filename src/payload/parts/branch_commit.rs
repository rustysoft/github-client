#[derive(Deserialize, Debug)]
pub struct BranchCommit {
    pub(in crate) sha: String,
}
