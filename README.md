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
