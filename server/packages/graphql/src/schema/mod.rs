use async_graphql::{EmptySubscription, Schema};
use mutation::MutationRoot;

use self::query::QueryRoot;

mod bucket;
mod folder;
mod manage_detail;
pub mod mutation;
mod object;
pub mod query;
mod user;

pub type RootSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
