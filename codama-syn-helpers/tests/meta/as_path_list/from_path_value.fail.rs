use codama_syn_helpers_test_macros::as_path_list;

#[as_path_list(foo = bar(1, 2, 3))]
pub struct Test;

fn main() {}