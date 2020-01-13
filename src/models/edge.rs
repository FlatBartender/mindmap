use crate::models::NodeRef;

pub struct Edge {
    pub brief: String,
    pub description: String,
    pub from: NodeRef,
    pub to: NodeRef,
    pub style: EdgeStyle,
}

pub enum EdgeStyle {
    Dotted,
    Full,
    Dashed,
}
