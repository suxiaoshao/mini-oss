use async_graphql::{EmptySubscription, Schema};

use self::root::{MutationRoot, QueryRoot};

pub mod root;
pub type RootSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
