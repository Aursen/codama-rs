use codama_korok_visitors::{KorokVisitable, MapVisitor};
use codama_koroks::StructKorok;
use codama_nodes::PublicKeyTypeNode;
use codama_syn_helpers::syn_build;
use quote::quote;

#[test]
fn it_can_set_a_node_on_all_koroks() {
    let ast: syn::ItemStruct = syn_build::parse(quote! { struct Foo(u32); });
    let mut korok = StructKorok::parse(&ast).unwrap();

    korok.accept(&mut MapVisitor::new(|k| {
        k.set_node(Some(PublicKeyTypeNode::new().into()))
    }));

    assert_eq!(korok.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(korok.fields.node, Some(PublicKeyTypeNode::new().into()));
    let field = &korok.fields.all[0];
    assert_eq!(field.node, Some(PublicKeyTypeNode::new().into()));
    assert_eq!(field.r#type.node, Some(PublicKeyTypeNode::new().into()));
}

#[test]
fn it_can_reset_all_nodes() {
    let ast: syn::ItemStruct = syn_build::parse(quote! { struct Foo(u32); });
    let mut korok = StructKorok::parse(&ast).unwrap();
    korok.node = Some(PublicKeyTypeNode::new().into());
    korok.fields.node = Some(PublicKeyTypeNode::new().into());
    let field = &mut korok.fields.all[0];
    field.node = Some(PublicKeyTypeNode::new().into());
    field.r#type.node = Some(PublicKeyTypeNode::new().into());

    korok.accept(&mut MapVisitor::new(|k| k.set_node(None)));

    assert_eq!(korok.node, None);
    assert_eq!(korok.fields.node, None);
    let field = &korok.fields.all[0];
    assert_eq!(field.node, None);
    assert_eq!(field.r#type.node, None);
}