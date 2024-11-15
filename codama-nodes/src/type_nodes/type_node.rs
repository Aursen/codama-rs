use codama_nodes_derive::IntoEnum;

use crate::{
    AmountTypeNode, ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode, EnumTypeNode,
    FixedSizeTypeNode, HiddenPrefixTypeNode, HiddenSuffixTypeNode, NumberTypeNode,
    PostOffsetTypeNode, PreOffsetTypeNode, SolAmountTypeNode, StringTypeNode, StructTypeNode,
    TupleTypeNode, TypeNodeEnumTrait,
};

#[derive(IntoEnum, Debug, PartialEq)]
pub enum TypeNode {
    Amount(AmountTypeNode),
    Array(Box<ArrayTypeNode>),
    Boolean(BooleanTypeNode),
    Bytes(BytesTypeNode),
    DateTime(DateTimeTypeNode),
    Enum(EnumTypeNode),
    FixedSize(Box<FixedSizeTypeNode<TypeNode>>),
    HiddenPrefix(Box<HiddenPrefixTypeNode<TypeNode>>),
    HiddenSuffix(Box<HiddenSuffixTypeNode<TypeNode>>),
    Number(NumberTypeNode),
    PostOffset(Box<PostOffsetTypeNode<TypeNode>>),
    PreOffset(Box<PreOffsetTypeNode<TypeNode>>),
    SolAmount(SolAmountTypeNode),
    String(StringTypeNode),
    Struct(StructTypeNode),
    Tuple(TupleTypeNode),
}

impl TypeNodeEnumTrait for TypeNode {}
