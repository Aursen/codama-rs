use crate::{
    ConstantDiscriminatorNode, FieldDiscriminatorNode, HasKind, NodeUnionTrait,
    SizeDiscriminatorNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum DiscriminatorNode {
    Constant(ConstantDiscriminatorNode),
    Field(FieldDiscriminatorNode),
    Size(SizeDiscriminatorNode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind() {
        let node: DiscriminatorNode = SizeDiscriminatorNode::new(42).into();
        assert_eq!(node.kind(), "sizeDiscriminatorNode");
    }
}
