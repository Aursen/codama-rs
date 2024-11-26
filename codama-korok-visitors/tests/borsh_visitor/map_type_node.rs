use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{
    BooleanTypeNode, MapTypeNode, Node, NumberTypeNode, PrefixedCountNode, U32, U64,
};
use quote::quote;

#[test]
fn it_identifies_map_types() {
    assert_eq!(
        get_node_from_type(quote! { Map<u64, bool> }),
        Some(Node::Type(
            MapTypeNode::new(
                NumberTypeNode::le(U64),
                BooleanTypeNode::default(),
                PrefixedCountNode::new(NumberTypeNode::le(U32))
            )
            .into()
        ))
    );
    assert!(matches!(
        get_node_from_type(quote! { std::iter::Map<u64, bool> }),
        Some(_)
    ));
    assert!(matches!(
        get_node_from_type(quote! { core::iter::Map<u64, bool> }),
        Some(_)
    ));
    assert_eq!(
        get_node_from_type(quote! { some::wrong::Map<u64, bool> }),
        None
    );
    assert_eq!(get_node_from_type(quote! { Map }), None);
    assert_eq!(get_node_from_type(quote! { Map<'a> }), None);
}
