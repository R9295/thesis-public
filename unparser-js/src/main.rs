#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod ast;
mod render;
use ast::Statement;
use libafl_fuzzer::{fuzz, impl_converter, impl_input};
use serde::{Deserialize, Serialize};
use thesis::Node;
#[derive(Serialize, Deserialize, thesis::Grammar, thesis::ToNautilus, Clone, Debug)]
pub struct Code {
    data: Vec<Statement>,
}

impl_converter!(Code, |data: Code| {
    if data.data.len() == 0 {
        "console".as_bytes().to_vec()
    } else {
        let res = data.data
            .iter()
            .map(|i| i.to_string())
            .collect::<String>();
/*         println!("{}", res); */
           res.as_bytes().to_vec()
    }
});

impl_input!(Code);

fn main() {
    /*     libafl_fuzzer::debug_grammar!(Code); */
    fuzz(FuzzDataTargetBytesConverter::new());
/*     println!("{}", Statement::to_nautilus()); */
}
