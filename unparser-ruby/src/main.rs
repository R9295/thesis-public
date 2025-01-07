mod ast;
mod render;
use std::{fs::create_dir, path::PathBuf};

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
    let trials = (1..11).map(|i| format!("trial-{}", i)).collect::<Vec<_>>();
    let base = PathBuf::from("/home/aarnav/projects/thesis/coverage/thesis-ruby-data/thesisphp0/experiment-folders/mruby_fuzz-thesis_ruby");
    let here = PathBuf::from("/home/aarnav/projects/thesis/unparser-ruby/results");
    for i in trials {
        let my_dir = create_dir(here.join(&i)).unwrap();
        let my_dir = here.join(&i);
        let path = base.join(&i).join("corpus").join("queue");
        println!("{:?}", path);
        let data = std::fs::read_dir(path).unwrap();
        for item in data {
            let path = item.unwrap().path();
            if path.extension().is_none() {
                let data = std::fs::read(&path).unwrap();
                let obj: Code = bincode::deserialize(&data).unwrap();
                let string = obj.data.iter().map(|i| i.to_string()).collect::<String>();
                std::fs::write(my_dir.join(format!("{}", path.file_name().unwrap().to_str().unwrap())), string).unwrap();
            }
        }
    }
}
