#![allow(warnings)]
#![feature(core_intrinsics)]
mod context;
mod feedback;
mod hooks;
mod mutators;
mod scheduler;
mod stages;
use clap::Parser;
use context::Context;
use feedback::register::RegisterFeedback;
use libafl::{
    corpus::{CachedOnDiskCorpus, Corpus, OnDiskCorpus},
    events::{ClientDescription, EventConfig, Launcher, SimpleEventManager},
    executors::ForkserverExecutor,
    feedback_or, feedback_or_fast,
    feedbacks::{
        CrashFeedback, MaxMapOneOrFilledFeedback, MaxMapPow2Feedback, TimeFeedback, TimeoutFeedback,
    },
    inputs::{Input, TargetBytesConverter},
    monitors::{MultiMonitor, SimpleMonitor},
    mutators::StdScheduledMutator,
    observers::{CanTrack, HitcountsMapObserver, StdMapObserver, TimeObserver},
    schedulers::{powersched::PowerSchedule, StdWeightedScheduler},
    stages::{IfStage, StdPowerMutationalStage},
    state::{HasCorpus, HasCurrentTestcase, StdState},
    Evaluator, Fuzzer, HasMetadata, StdFuzzer,
};
use libafl_bolts::{
    core_affinity::{CoreId, Cores},
    current_nanos,
    fs::get_unique_std_input_file,
    ownedref::OwnedRefMut,
    rands::{RomuDuoJrRand, StdRand},
    shmem::{ShMem, ShMemProvider, StdShMemProvider, UnixShMemProvider},
    tuples::{tuple_list, Handled},
    AsSliceMut, Error,
};
use libafl_targets::{AFLppCmpLogMap, AFLppCmpLogObserver};
use mutators::{
    recurse_mutate::ThesisRecurseMutator, splice::ThesisSpliceMutator,
    splice_append::ThesisSpliceAppendMutator,
};

#[cfg(feature = "scale")]
use parity_scale_codec::{Decode, Encode};

use regex::Regex;
use stages::{
    deterministic::DeterministicStage, generate::GenerateStage, minimization::MinimizationStage,
    recursive_minimization::RecursiveMinimizationStage,
};
use std::{cell::RefCell, io::ErrorKind, path::PathBuf, process::Command, rc::Rc, time::Duration};
use thesis::{DepthInfo, Node, Visitor};

use crate::stages::generate::generate;

const SHMEM_ENV_VAR: &str = "__AFL_SHM_ID";
pub fn fuzz<I, TC>(bytes_converter: TC)
where
    I: Node + Input,
    TC: TargetBytesConverter<Input = I> + Clone,
{
    let monitor = MultiMonitor::new(|s| println!("{s}"));
    /*     let monitor = MultiMonitor::new(|s| {}); */
    let mut mgr = SimpleEventManager::new(monitor);
    let opt = Opt::parse();
    if !opt.output_dir.exists() {
        std::fs::create_dir(&opt.output_dir).unwrap();
    }
    let shmem_provider = StdShMemProvider::new().expect("Failed to init shared memory");
    let broker_port = 7777;
    let map_size = Command::new(opt.executable.clone())
        .env("AFL_DUMP_MAP_SIZE", "1")
        .output()
        .expect("target gave no output");
    let map_size = String::from_utf8(map_size.stdout)
        .expect("target returned illegal mapsize")
        .replace("\n", "");
    let map_size = map_size.parse::<usize>().expect("illegal mapsize output") + opt.map_bias;
    let fuzzer_dir = opt.output_dir;
    match std::fs::create_dir(&fuzzer_dir) {
        Ok(_) => {}
        Err(e) => {
            if matches!(e.kind(), ErrorKind::AlreadyExists) {
            } else {
                panic!("{:?}", e)
            }
        }
    };
    // Create the shared memory map for comms with the forkserver
    let mut shmem_provider = UnixShMemProvider::new().unwrap();
    let mut shmem = shmem_provider.new_shmem(map_size).unwrap();
    shmem.write_to_env(SHMEM_ENV_VAR).unwrap();
    let shmem_buf = shmem.as_slice_mut();

    // Create an observation channel to keep track of edges hit.
    let edges_observer = unsafe {
        HitcountsMapObserver::new(StdMapObserver::new("edges", shmem_buf)).track_indices()
    };
    let seed = opt.rng_seed.unwrap_or(current_nanos());

    let mut visitor = Visitor::new(
        seed,
        DepthInfo {
            expand: 1500,
            generate: 3,
            iterate: 5,
        },
    );
    let visitor = Rc::new(RefCell::new(visitor));
    // Create a MapFeedback for coverage guided fuzzin'
    // We only care if an edge was hit, not how many times
    let map_feedback = MaxMapOneOrFilledFeedback::new(&edges_observer);

    // Create an observation channel to keep track of the execution time.
    let time_observer = TimeObserver::new("time");
    let minimization_stage = MinimizationStage::new(Rc::clone(&visitor), &map_feedback);
    let recursive_minimization_stage =
        RecursiveMinimizationStage::new(Rc::clone(&visitor), &map_feedback);
    let mut feedback = feedback_or!(
        map_feedback,
        TimeFeedback::new(&time_observer),
        RegisterFeedback::new()
    );

    let mut objective = feedback_or_fast!(CrashFeedback::new());

    // Initialize our State if necessary
    let mut state = StdState::new(
        RomuDuoJrRand::with_seed(seed),
        // TODO: configure testcache size
        CachedOnDiskCorpus::<I>::new(fuzzer_dir.join("queue"), 2).unwrap(),
        OnDiskCorpus::<I>::new(fuzzer_dir.join("crash")).unwrap(),
        &mut feedback,
        &mut objective,
    )
    .unwrap();
    if !fuzzer_dir.join("chunks").exists() {
        std::fs::create_dir(fuzzer_dir.join("chunks")).unwrap();
    }
    if !fuzzer_dir.join("cmps").exists() {
        std::fs::create_dir(fuzzer_dir.join("cmps")).unwrap();
    }
    let context = Context::new(fuzzer_dir.clone());
    state.add_metadata(context);

    let scheduler = StdWeightedScheduler::with_schedule(
        &mut state,
        &edges_observer,
        Some(PowerSchedule::explore()),
    );
    let scheduler = scheduler.cycling_scheduler();
    let mut executor = ForkserverExecutor::builder()
        .program(opt.executable.clone())
        .coverage_map_size(map_size)
        .debug_child(opt.debug_child)
        .is_persistent(true)
        .is_deferred_frksrv(true)
        .timeout(Duration::from_millis(opt.hang_timeout))
        .shmem_provider(&mut shmem_provider)
        .target_bytes_converter(bytes_converter.clone())
        .build(tuple_list!(edges_observer, time_observer))
        .unwrap();

    // Create our Fuzzer
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);
    if let Some(dict_file) = &opt.dict_file {
        let file = std::fs::read_to_string(dict_file).expect("cannot read dict file");
        for entry in file.split("\n") {
            visitor.borrow_mut().register_string(entry.to_string());
        }
    }
    if opt.get_strings {
        let string_regex = Regex::new("^[a-zA-Z0-9_]+$").unwrap();
        let strings = Command::new("strings")
            .arg(opt.executable.clone())
            .output()
            .expect("strings gave no output!");
        let strings = String::from_utf8_lossy(&strings.stdout);
        for string in strings.lines().into_iter() {
            if string_regex.is_match(string) {
                visitor.borrow_mut().register_string(string.to_string());
            }
        }
    }
    if state.must_load_initial_inputs() {
        for _ in 0..opt.initial_generated_inputs {
            let generated: I = generate(&mut visitor.borrow_mut());
            fuzzer
                .evaluate_input(&mut state, &mut executor, &mut mgr, generated)
                .unwrap();
        }
        println!("We imported {} inputs from disk.", state.corpus().count());
    }

    let mutator = StdScheduledMutator::with_max_stack_pow(
        tuple_list!(
            // SPLICE
            ThesisSpliceMutator::new(Rc::clone(&visitor)),
            ThesisSpliceMutator::new(Rc::clone(&visitor)),
            // RECURSIVE GENERATE
            ThesisRecurseMutator::new(Rc::clone(&visitor)),
            ThesisRecurseMutator::new(Rc::clone(&visitor)),
            // SPLICE APPEND
            ThesisSpliceAppendMutator::new(Rc::clone(&visitor)),
        ),
        3,
    );

    let mut stages = tuple_list!(
        // we mut minimize before calculating testcase score
        minimization_stage,
        recursive_minimization_stage,
        StdPowerMutationalStage::new(mutator),
        /*         GenerateStage::new(Rc::clone(&visitor)), */
    );

    fuzzer
        .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
        .unwrap();
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser, Clone)]
#[command(
    name = "thesis",
    about = "thesis",
    author = "aarnav <aarnavbos@gmail.com>"
)]
struct Opt {
    executable: PathBuf,
    #[arg(short = 'o')]
    output_dir: PathBuf,
    /// Timeout in ms
    #[arg(short = 't', default_value_t = 1000)]
    hang_timeout: u64,

    /// seed for rng
    #[arg(short = 's')]
    rng_seed: Option<u64>,

    #[arg(short = 'd')]
    debug_child: bool,

    #[arg(short = 'm')]
    map_bias: usize,

    #[arg(short = 'g', default_value_t = 100)]
    initial_generated_inputs: usize,

    #[arg(short = 'c', value_parser=Cores::from_cmdline)]
    cores: Cores,

    #[arg(short = 'x')]
    dict_file: Option<PathBuf>,

    #[arg(short = 'e')]
    cmplog: bool,

    #[arg(short = 'S')]
    get_strings: bool,
}

#[macro_export]
macro_rules! debug_grammar {
    ($t:ty) => {
        use thesis::Visitor;
        let mut v = Visitor::new(
            libafl_bolts::current_nanos(),
            thesis::DepthInfo {
                expand: 1500,
                generate: 5,
                iterate: 3,
            },
        );
        let gen_depth = v.generate_depth();
        for _ in 0..100 {
            println!(
                "{}",
                <$t>::generate(&mut v, &mut gen_depth.clone(), &mut 0)
                    .data
                    .iter()
                    .map(|i| format!("{}\n", i))
                    .collect::<String>()
            );
            println!("--------------------------------");
        }
    };
}

#[macro_export]
macro_rules! impl_converter {
    ($t:ty) => {
        #[derive(Clone)]
        struct FuzzDataTargetBytesConverter;

        impl FuzzDataTargetBytesConverter {
            fn new() -> Self {
                Self {}
            }
        }

        impl libafl::inputs::TargetBytesConverter for FuzzDataTargetBytesConverter {
            type Input = $t;

            fn to_target_bytes<'a>(
                &mut self,
                input: &'a Self::Input,
            ) -> libafl_bolts::ownedref::OwnedSlice<'a, u8> {
                let bytes = thesis::serialize(&input);
                libafl_bolts::ownedref::OwnedSlice::from(bytes)
            }
        }
    };
    ($t:ty, $closure:expr) => {
        #[derive(Clone)]
        struct FuzzDataTargetBytesConverter;

        impl FuzzDataTargetBytesConverter {
            fn new() -> Self {
                Self
            }
        }

        impl libafl::inputs::TargetBytesConverter for FuzzDataTargetBytesConverter {
            type Input = $t;

            fn to_target_bytes<'a>(
                &mut self,
                input: &'a Self::Input,
            ) -> libafl_bolts::ownedref::OwnedSlice<'a, u8> {
                libafl_bolts::ownedref::OwnedSlice::from($closure(input.clone()))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_input {
    ($t:ty) => {
        impl libafl::inputs::Input for $t {
            fn to_file<P>(&self, path: P) -> Result<(), libafl::Error>
            where
                P: AsRef<std::path::Path>,
            {
                let bytes = thesis::serialize(self);
                std::fs::write(path, bytes)?;
                Ok(())
            }
            // TODO: don't serialize here
            fn generate_name(&self, id: Option<libafl::corpus::CorpusId>) -> String {
                let bytes = thesis::serialize(self);
                format!("{}", blake3::hash(bytes.as_slice()))
            }

            fn from_file<P>(path: P) -> Result<Self, libafl::Error>
            where
                P: AsRef<std::path::Path>,
            {
                let data = std::fs::read(path)?;
                let res = thesis::deserialize::<$t>(&mut data.as_slice());
                Ok(res)
            }
        }
    };
}
