use codama_syn_helpers_test_macros::as_path;

#[as_path(foo(1, 2, 3))]
pub struct Test;

#[as_path(foo = (1, 2, 3))]
pub struct TestWithEq;

fn main() {}