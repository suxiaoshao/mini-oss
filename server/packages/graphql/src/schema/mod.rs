use async_graphql::{EmptySubscription, Schema};

use self::root::{MutationRoot, QueryRoot};

mod bucket;
mod folder;
pub mod root;
mod user_info;
mod object;

pub type RootSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
