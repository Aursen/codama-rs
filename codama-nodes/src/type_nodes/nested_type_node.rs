use crate::{
    FixedSizeTypeNode, HiddenPrefixTypeNode, HiddenSuffixTypeNode, NestedTypeNodeTrait, NodeTrait,
    NodeUnionTrait, PostOffsetTypeNode, PreOffsetTypeNode, SentinelTypeNode, SizePrefixTypeNode,
    TypeNodeTrait, TypeNodeUnionTrait,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum NestedTypeNode<T: TypeNodeTrait> {
    FixedSize(Box<FixedSizeTypeNode<NestedTypeNode<T>>>),
    HiddenPrefix(Box<HiddenPrefixTypeNode<NestedTypeNode<T>>>),
    HiddenSuffix(Box<HiddenSuffixTypeNode<NestedTypeNode<T>>>),
    PostOffset(Box<PostOffsetTypeNode<NestedTypeNode<T>>>),
    PreOffset(Box<PreOffsetTypeNode<NestedTypeNode<T>>>),
    Sentinel(Box<SentinelTypeNode<NestedTypeNode<T>>>),
    SizePrefix(Box<SizePrefixTypeNode<NestedTypeNode<T>>>),
    #[fallback]
    Value(T),
}

impl<T: TypeNodeTrait> TypeNodeUnionTrait for NestedTypeNode<T> {}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for NestedTypeNode<T> {
    fn get_nested_type_node(&self) -> &T {
        match self {
            NestedTypeNode::FixedSize(node) => node.get_nested_type_node(),
            NestedTypeNode::HiddenPrefix(node) => node.get_nested_type_node(),
            NestedTypeNode::HiddenSuffix(node) => node.get_nested_type_node(),
            NestedTypeNode::PostOffset(node) => node.get_nested_type_node(),
            NestedTypeNode::PreOffset(node) => node.get_nested_type_node(),
            NestedTypeNode::Sentinel(node) => node.get_nested_type_node(),
            NestedTypeNode::SizePrefix(node) => node.get_nested_type_node(),
            NestedTypeNode::Value(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NodeUnionTrait, StringTypeNode};

    #[test]
    fn kind() {
        let node: NestedTypeNode<StringTypeNode> = StringTypeNode::utf8().into();
        assert_eq!(node.kind(), "stringTypeNode");
    }
}
