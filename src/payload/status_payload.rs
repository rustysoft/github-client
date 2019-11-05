#[derive(Deserialize, Debug)]
pub struct StatusPayload {
    state: State,
}

impl StatusPayload {
    pub fn state(&self) -> &State {
        &self.state
    }
}

#[derive(Deserialize, Debug)]
pub enum State {
    Failure,
    Pending,
    Success
}
