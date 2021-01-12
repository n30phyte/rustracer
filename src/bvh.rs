use super::models::Model;
use std::sync::Arc;

// TODO: Rewrite with https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/ in mind

pub struct BvhNode {
    left: Arc<dyn Model>,
    right: Arc<dyn Model>,
}

struct Arena {
    nodes: Vec<BvhNode>,
}
