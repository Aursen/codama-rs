use crate::{
    ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode, EnumValueNode,
    MapEntryValueNode, MapValueNode, NodeTrait, NodeUnionTrait, NoneValueNode, NumberValueNode,
    PublicKeyValueNode, SetValueNode, SomeValueNode, StringValueNode, StructFieldValueNode,
    StructValueNode, TupleValueNode,
};
use codama_nodes_derive::{node_union, RegisteredNodes};

#[derive(RegisteredNodes)]
#[node_union]
pub enum RegisteredValueNode {
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(Box<ConstantValueNode>),
    Enum(EnumValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    Number(NumberValueNode),
    PublicKey(PublicKeyValueNode),
    Set(SetValueNode),
    Some(Box<SomeValueNode>),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),

    #[registered]
    StructField(StructFieldValueNode),
    #[registered]
    MapEntry(MapEntryValueNode),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeUnionTrait;

    #[test]
    fn kind_from_standalone() {
        let node: ValueNode = NoneValueNode::new().into();
        assert_eq!(node.kind(), "noneValueNode");
    }

    #[test]
    fn kind_from_registered() {
        let node: RegisteredValueNode = NoneValueNode::new().into();
        assert_eq!(node.kind(), "noneValueNode");
    }
}
