use crate::models::NodeRef;
use crate::models::color::Color;

pub struct Node {
    pub text: String,
    pub color: Color,
    pub tags: Vec<String>,
    pub parent: Option<NodeRef>,
}
