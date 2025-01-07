use std::{fs::create_dir, path::{Path, PathBuf}};

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
        let res = data.data.iter().map(|i| i.to_string()).collect::<String>();
        /*         println!("{}", res); */
        res.as_bytes().to_vec()
    }
});

impl_input!(Code);

fn main() {
    let trials = (1..11).map(|i| format!("trial-{}", i)).collect::<Vec<_>>();
    let base = PathBuf::from("/home/aarnav/projects/thesis/coverage/thesis-js-data/thesisjs/experiment-folders/jerryscript_fuzz-thesis_js/");
    let here = PathBuf::from("/home/aarnav/projects/thesis/unparser-js/results");
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
