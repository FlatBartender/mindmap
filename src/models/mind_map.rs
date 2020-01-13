use slotmap::*;
use crate::models::node::Node;
use crate::models::history::History;
use crate::models::comment::Comment;
use crate::models::edge::Edge;
use crate::models::permission::Permission;
use crate::utils::genmap::GenMap;

new_key_type! {
    pub struct NodeRef;
    pub struct CommentRef;
    pub struct EdgeRef;
    pub struct PermissionRef;
}

pub struct MindMap {
    pub name:               String,
    pub default_permission: Permission,
    pub history:            Vec<History>,
    pub nodes:       GenMap<NodeRef, Node>,
    pub comments:    GenMap<CommentRef, Comment>,
    pub edges:       GenMap<EdgeRef, Edge>,
    pub permissions: GenMap<PermissionRef, Permission>,

}

impl MindMap {

}
