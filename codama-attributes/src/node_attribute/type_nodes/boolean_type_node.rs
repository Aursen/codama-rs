use crate::{utils::SetOnce, NodeAttributeParse};
use codama_nodes::{BooleanTypeNode, NestedTypeNode, Node, NumberTypeNode};
use codama_syn_helpers::syn_traits::*;

impl NodeAttributeParse for BooleanTypeNode {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> syn::Result<Node> {
        let mut size = SetOnce::<NestedTypeNode<NumberTypeNode>>::new("size");
        if meta.input.is_end_of_arg() || meta.input.is_empty_group() {
            meta.input.consume_arg()?;
            return Ok(BooleanTypeNode::default().into());
        }
        meta.parse_nested_meta(|ref meta| match meta.path.last_str().as_str() {
            "size" => todo!(),
            _ => {
                let node = Node::from_meta(meta)?;
                let node = match NestedTypeNode::<NumberTypeNode>::try_from(node) {
                    Ok(node) => node,
                    Err(_) => return Err(meta.error("size must be a NumberTypeNode")),
                };
                size.set(node, meta)?;
                Ok(())
            }
        })?;
        Ok(BooleanTypeNode::new(size.take(meta)?).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_node, assert_node_err, NodeAttribute};
    use codama_nodes::NumberFormat::U32;
    use codama_syn_helpers::syn_build;
    use quote::quote;

    #[test]
    fn ok() {
        assert_node!(#[node(boolean_type)], BooleanTypeNode::default().into());
        assert_node!(#[node(boolean_type())], BooleanTypeNode::default().into());
    }

    #[test]
    fn custom_size() {
        assert_node!(#[node(boolean_type(number_type(u32, le)))], BooleanTypeNode::new(NumberTypeNode::le(U32)).into());
    }

    #[test]
    fn unrecognized_node() {
        assert_node_err!(#[node(boolean_type(unrecognized))], "unrecognized node");
        assert_node_err!(#[node(boolean_type(foo = 42))], "unrecognized node");
    }
}