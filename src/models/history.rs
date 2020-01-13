use std::time::SystemTime;
use crate::models::{
    mind_map::{NodeRef, EdgeRef, PermissionRef, CommentRef, MindMap},
    color::Color,
    edge::{Edge, EdgeStyle},
    node::Node,
    comment::Comment,
    permission::{Permission, PermissionType},
    UserRef,
};

pub struct History {
    pub time: SystemTime,
    pub event: Event,
    pub user: UserRef,
}

pub enum Event {
    MindMapCreated(MindMap),
    MindMapModified(MindMapEvent),

    NodeAdded(Node),
    NodeDeleted(NodeRef),
    NodeModified(NodeRef, NodeEvent),

    EdgeAdded(Edge),
    EdgeDeleted(EdgeRef),
    EdgeModified(EdgeRef, EdgeEvent),

    PermissionAdded(Permission),
    PermissionDeleted(PermissionRef),
    PermissionModified(PermissionRef, PermissionEvent),

    CommentAdded(Comment),
    CommentDeleted(CommentRef),
    CommentModified(CommentRef, CommentEvent),
}

pub enum MindMapEvent {
    Name(String),
    DefaultPermission(Permission),
}

pub enum NodeEvent {
    Moved(NodeRef), // Parent changed to NodeRef
    Text(String),
    Color(Color),
    TagAdded(String),
    TagDeleted(String),
}

pub enum EdgeEvent {
    Brief(String),
    Description(String),
    Color(Color),
    Style(EdgeStyle),
}

pub enum PermissionEvent {
    Type(PermissionType),
}

pub enum CommentEvent {
    Message(String),
}
