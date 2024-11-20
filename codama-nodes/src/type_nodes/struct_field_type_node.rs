use crate::{CamelCaseString, DefaultValueStrategy, Docs, TypeNode, ValueNode};
use codama_nodes_derive::Node;

#[derive(Node, Debug, PartialEq, Clone)]
pub struct StructFieldTypeNode {
    // Data.
    pub name: CamelCaseString,
    pub default_value_strategy: Option<DefaultValueStrategy>,
    pub docs: Docs,

    // Children.
    pub r#type: TypeNode,
    pub default_value: Option<ValueNode>,
}

impl StructFieldTypeNode {
    pub fn new<T, U>(name: T, r#type: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            name: name.into(),
            default_value_strategy: None,
            docs: Docs::default(),
            r#type: r#type.into(),
            default_value: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberTypeNode, NumberValueNode, U32};

    #[test]
    fn new() {
        let node = StructFieldTypeNode::new("my_field", NumberTypeNode::le(U32));
        assert_eq!(node.name, CamelCaseString::new("myField"));
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
    }

    #[test]
    fn direct_instantiation() {
        let node = StructFieldTypeNode {
            name: "myField".into(),
            default_value_strategy: Some(DefaultValueStrategy::Optional),
            docs: vec!["Hello".to_string()].into(),
            r#type: NumberTypeNode::le(U32).into(),
            default_value: Some(NumberValueNode::new(42u32).into()),
        };

        assert_eq!(node.name, CamelCaseString::new("myField"));
        assert_eq!(
            node.default_value_strategy,
            Some(DefaultValueStrategy::Optional)
        );
        assert_eq!(*node.docs, vec!["Hello".to_string()]);
        assert_eq!(node.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
        assert_eq!(node.default_value, Some(NumberValueNode::new(42u32).into()));
    }
}
