#![allow(warnings)]
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
    [
        "<?php\n".to_string().as_bytes().to_vec(),
        data.data
            .iter()
            .map(|i| i.to_string())
            .collect::<String>()
            .as_bytes()
            .to_vec(),
        "\n?>".to_string().as_bytes().to_vec(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
});

impl_input!(Code);

fn main() {
    fuzz(FuzzDataTargetBytesConverter::new());
}
