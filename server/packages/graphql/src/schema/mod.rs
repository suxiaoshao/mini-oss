use async_graphql::{EmptySubscription, Schema};

use self::root::{MutationRoot, QueryRoot};

mod bucket;
pub mod root;
mod user_info;
pub type RootSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
