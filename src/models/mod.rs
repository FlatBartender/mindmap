pub mod edge;
pub mod node;
pub mod mind_map;
pub mod history;
pub mod permission;
pub mod user;
pub mod color;
pub mod comment;

use slotmap::new_key_type;

pub use mind_map::{NodeRef, HistoryRef, CommentRef, EdgeRef};

new_key_type! {
    pub struct UserRef;
}
