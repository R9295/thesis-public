mod ast;
mod render;
use ast::Statement;
use libafl_fuzzer::{fuzz, impl_converter, impl_input};
use serde::{Deserialize, Serialize};
use thesis::Node;
#[derive(Serialize, Deserialize, thesis::Grammar, Clone, Debug)]
pub struct Code {
    data: Vec<Statement>,
}

impl_converter!(Code, |data: Code| {
    if data.data.len() == 0 {
        "exit".as_bytes().to_vec()
    } else {
        data.data
            .iter()
            .map(|i| i.to_string())
            .collect::<String>()
            .as_bytes()
            .to_vec()
    }
});

impl_input!(Code);

fn main() {
    fuzz(FuzzDataTargetBytesConverter::new());
}
