use codama_nodes_derive::node;

#[node]
pub struct PayerValueNode {}

impl PayerValueNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = PayerValueNode::new();
        assert_eq!(node, PayerValueNode {});
    }
}
