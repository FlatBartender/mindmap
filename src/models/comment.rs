use crate::models::{
    mind_map::NodeRef,
    UserRef,
};

#[derive(Clone)]
pub struct Comment {
    node: NodeRef,
    message: String,
    from: UserRef,
}
