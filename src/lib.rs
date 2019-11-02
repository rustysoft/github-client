#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod client;
mod merge_method;
pub mod payload;

pub use self::client::GithubClient;
pub use self::merge_method::MergeMethod;
