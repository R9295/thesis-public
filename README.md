Steps to reproduce (Linux ONLY)

1. install FuzzBench dependencies
- docker
- llvm-18 (https://apt.llvm.org/)
- python3.10
- the python depdendencies in ``evaluation/fuzzbench/requirements.txt``

2. Install the latest rust nightly
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustup default nightly
```

3. copy thesis code
```
cd ./evaluation/fuzzbench
./copy.sh
```

4. run the evaluation

First create the directory for the results
```
mkdir /tmp/fuzzbench-results
```
Run this command for each fuzzer and fuzzing target. Each evaluation takes one day.
``` bash
# replace <TARGET> with mruby_fuzz / php_fuzz / jerryscript_fuzz
# replace <FUZZER> with gramatron / nautilus / thesis_php (only php) / thesis_js (only jerryscript) / thesis_ruby (only mruby)
PYTHONPATH=. python3 experiment/run_experiment.py \
                   --experiment-config thesis/config.yaml \
                   --benchmarks <TARGET> \
                   --experiment-name <TARGET>_<FUZZER>_0
                   --fuzzers <FUZZER> --concurrent-builds 1 -a
```
if you get stuck: refer to the [FuzzBench docs](https://google.github.io/fuzzbench/running-a-local-experiment)

5. analyze coverage

build all coverage runners
```
cd evaluation/fuzzbench
make debug-custom_cov-php_fuzz
make debug-custom_cov-mruby_fuzz
make debug-custom_cov-jerryscript_fuzz
```
7. Copy all coverage runners from the docker container.
```
docker run --rm --entrypoint cat gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz:latest /out/fuzz > ./mruby-cov
docker run --rm --entrypoint cat gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz:latest /out/fuzz > ./jerrscript-cov
docker run --rm --entrypoint cat gcr.io/fuzzbench/builders/custom_cov/php_fuzz:latest /out/fuzz > ./php-cov
```
8. Extract the results
```
cd evaluation/fuzzbench/coverage
python3 get_results.py /tmp/fuzzbench-results/
```
9. get the coverage for each trial for each fuzzer's corpus

```
LLVM_PROFILE_FILE="<FUZZER>.<TARGET>.<TRIAL>.profraw" ./<FUZZER>-cov \
      -timeout=0.2  \
      -print_coverage=1 \
      -fork=1\
      -ignore_timeouts=1\
      -ignore_crashes=1\
      -ignore_ooms=1\
      -rss_limit_mb=1024 \
      -runs=0 \
      <CORPUS_FOLDER>
```
10. translate the ``profraw`` to ``profdata``

```
llvm-profdata-18 merge -sparse <FUZZER>.<TARGET>.<TRIAL>.profraw -o <FUZZER>.<TARGET>.<TRIAL>.profdata
```
11. get the coverage

This will give you the coverage of the target in html. There you will see the median branch coverage for all targets, intrepreter and parser.
```
llvm-cov-18 show -format=html -instr-profile=<FUZZER>.<TARGET>.<TRIAL>.profdata  ./<FUZZER>-cov -output-dir=<COVERAGE_OUTPUT>
```

