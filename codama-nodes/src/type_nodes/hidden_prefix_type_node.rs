use crate::{
    ConstantValueNode, NestedTypeNode, NestedTypeNodeTrait, TypeNode, TypeNodeTrait,
    TypeNodeUnionTrait,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes_derive::nestable_type_node;

#[nestable_type_node]
pub struct HiddenPrefixTypeNode<T: TypeNodeUnionTrait> {
    // Children.
    #[serde(bound = "T: TypeNodeUnionTrait")]
    pub r#type: Box<T>,
    pub prefix: Vec<ConstantValueNode>,
}

impl From<HiddenPrefixTypeNode<crate::TypeNode>> for crate::Node {
    fn from(val: HiddenPrefixTypeNode<crate::TypeNode>) -> Self {
        crate::Node::Type(val.into())
    }
}

impl<T: TypeNodeTrait> From<HiddenPrefixTypeNode<NestedTypeNode<T>>>
    for HiddenPrefixTypeNode<TypeNode>
{
    fn from(node: HiddenPrefixTypeNode<NestedTypeNode<T>>) -> Self {
        HiddenPrefixTypeNode {
            r#type: Box::new(TypeNode::from(*node.r#type)),
            prefix: node.prefix,
        }
    }
}

impl<T: TypeNodeTrait> TryFrom<HiddenPrefixTypeNode<TypeNode>>
    for HiddenPrefixTypeNode<NestedTypeNode<T>>
{
    type Error = CodamaError;
    fn try_from(node: HiddenPrefixTypeNode<TypeNode>) -> CodamaResult<Self> {
        Ok(HiddenPrefixTypeNode {
            r#type: Box::new(NestedTypeNode::try_from(*node.r#type)?),
            prefix: node.prefix,
        })
    }
}

impl<T: TypeNodeUnionTrait> HiddenPrefixTypeNode<T> {
    pub fn new<U>(r#type: U, prefix: Vec<ConstantValueNode>) -> Self
    where
        U: Into<T>,
    {
        Self {
            r#type: Box::new(r#type.into()),
            prefix,
        }
    }
}

impl<T: TypeNodeTrait> NestedTypeNodeTrait<T> for HiddenPrefixTypeNode<NestedTypeNode<T>> {
    type Mapped<U: TypeNodeTrait> = HiddenPrefixTypeNode<NestedTypeNode<U>>;

    fn get_nested_type_node(&self) -> &T {
        self.r#type.get_nested_type_node()
    }

    fn try_map_nested_type_node<U: TypeNodeTrait, F: FnOnce(T) -> CodamaResult<U>>(
        self,
        f: F,
    ) -> CodamaResult<Self::Mapped<U>> {
        Ok(HiddenPrefixTypeNode {
            r#type: Box::new(self.r#type.try_map_nested_type_node(f)?),
            prefix: self.prefix,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Base16, NestedTypeNode, StringTypeNode, TypeNode};

    #[test]
    fn new_type_node() {
        let node = HiddenPrefixTypeNode::<TypeNode>::new(
            StringTypeNode::utf8(),
            vec![ConstantValueNode::bytes(Base16, "ffff")],
        );
        assert_eq!(*node.r#type, TypeNode::String(StringTypeNode::utf8()));
        assert_eq!(node.prefix, vec![ConstantValueNode::bytes(Base16, "ffff")]);
    }

    #[test]
    fn new_nested_type_node() {
        let node = HiddenPrefixTypeNode::<NestedTypeNode<StringTypeNode>>::new(
            StringTypeNode::utf8(),
            vec![],
        );
        assert_eq!(*node.r#type, NestedTypeNode::Value(StringTypeNode::utf8()));
        assert_eq!(node.get_nested_type_node(), &StringTypeNode::utf8());
        assert_eq!(node.prefix, vec![]);
    }

    #[test]
    fn to_json() {
        let node = HiddenPrefixTypeNode::<TypeNode>::new(
            StringTypeNode::utf8(),
            vec![ConstantValueNode::bytes(Base16, "ffff")],
        );
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"hiddenPrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":[{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}]}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"hiddenPrefixTypeNode","type":{"kind":"stringTypeNode","encoding":"utf8"},"prefix":[{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"ffff","encoding":"base16"}}]}"#;
        let node: HiddenPrefixTypeNode<TypeNode> = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            HiddenPrefixTypeNode::<TypeNode>::new(
                StringTypeNode::utf8(),
                vec![ConstantValueNode::bytes(Base16, "ffff")],
            )
        );
    }
}
