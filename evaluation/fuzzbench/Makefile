export DOCKER_BUILDKIT := 1

jerryscript_fuzz-fuzz-target=fuzz

mruby_fuzz-fuzz-target=fuzz

php_fuzz-fuzz-target=fuzz

base-image:
	docker pull ubuntu:focal
	docker build \
	--tag gcr.io/fuzzbench/base-image \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/base-image \
	--file docker/base-image/Dockerfile \
	.

dispatcher-image: base-image
	docker build \
	--tag gcr.io/fuzzbench/dispatcher-image \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/dispatcher-image \
	--file docker/dispatcher-image/Dockerfile \
	docker/dispatcher-image

worker: base-image
	docker build \
	--tag gcr.io/fuzzbench/worker \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/worker \
	--file docker/worker/Dockerfile \
	.

.jerryscript_fuzz-project-builder:
	docker build \
	--tag gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file benchmarks/jerryscript_fuzz/Dockerfile \
	benchmarks/jerryscript_fuzz

.coverage-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/coverage/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/coverage/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/coverage/builder.Dockerfile \
	fuzzers/coverage

.coverage-jerryscript_fuzz-builder: .coverage-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/coverage/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/coverage/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=coverage \
	--build-arg parent_image=gcr.io/fuzzbench/builders/coverage/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/afl/builder.Dockerfile \
	fuzzers/afl

.afl-jerryscript_fuzz-builder-debug: .afl-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/jerryscript_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=afl \
	--build-arg parent_image=gcr.io/fuzzbench/builders/afl/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-jerryscript_fuzz-builder: .afl-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=afl \
	--build-arg parent_image=gcr.io/fuzzbench/builders/afl/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-jerryscript_fuzz-intermediate-runner: .afl-jerryscript_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/afl/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/afl/jerryscript_fuzz-intermediate \
	--file fuzzers/afl/runner.Dockerfile \
	fuzzers/afl

.afl-jerryscript_fuzz-runner: .afl-jerryscript_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/afl/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/afl/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=afl \
	--file docker/benchmark-runner/Dockerfile \
	.

run-afl-jerryscript_fuzz: .afl-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/afl/jerryscript_fuzz

debug-afl-jerryscript_fuzz: .afl-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/afl/jerryscript_fuzz

test-run-afl-jerryscript_fuzz: .afl-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/afl/jerryscript_fuzz

debug-builder-afl-jerryscript_fuzz: .afl-jerryscript_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/afl/jerryscript_fuzz

.mruby_fuzz-project-builder:
	docker build \
	--tag gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file benchmarks/mruby_fuzz/Dockerfile \
	benchmarks/mruby_fuzz

.coverage-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/coverage/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/coverage/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/coverage/builder.Dockerfile \
	fuzzers/coverage

.coverage-mruby_fuzz-builder: .coverage-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/coverage/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/coverage/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=coverage \
	--build-arg parent_image=gcr.io/fuzzbench/builders/coverage/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/afl/builder.Dockerfile \
	fuzzers/afl

.afl-mruby_fuzz-builder-debug: .afl-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/mruby_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=afl \
	--build-arg parent_image=gcr.io/fuzzbench/builders/afl/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-mruby_fuzz-builder: .afl-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=afl \
	--build-arg parent_image=gcr.io/fuzzbench/builders/afl/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-mruby_fuzz-intermediate-runner: .afl-mruby_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/afl/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/afl/mruby_fuzz-intermediate \
	--file fuzzers/afl/runner.Dockerfile \
	fuzzers/afl

.afl-mruby_fuzz-runner: .afl-mruby_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/afl/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/afl/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=afl \
	--file docker/benchmark-runner/Dockerfile \
	.

run-afl-mruby_fuzz: .afl-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/afl/mruby_fuzz

debug-afl-mruby_fuzz: .afl-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/afl/mruby_fuzz

test-run-afl-mruby_fuzz: .afl-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/afl/mruby_fuzz

debug-builder-afl-mruby_fuzz: .afl-mruby_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/afl/mruby_fuzz

.php_fuzz-project-builder:
	docker build \
	--tag gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file benchmarks/php_fuzz/Dockerfile \
	benchmarks/php_fuzz

.coverage-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/coverage/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/coverage/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/coverage/builder.Dockerfile \
	fuzzers/coverage

.coverage-php_fuzz-builder: .coverage-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/coverage/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/coverage/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=coverage \
	--build-arg parent_image=gcr.io/fuzzbench/builders/coverage/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/afl/builder.Dockerfile \
	fuzzers/afl

.afl-php_fuzz-builder-debug: .afl-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/php_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=afl \
	--build-arg parent_image=gcr.io/fuzzbench/builders/afl/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-php_fuzz-builder: .afl-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/afl/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/afl/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=afl \
	--build-arg parent_image=gcr.io/fuzzbench/builders/afl/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.afl-php_fuzz-intermediate-runner: .afl-php_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/afl/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/afl/php_fuzz-intermediate \
	--file fuzzers/afl/runner.Dockerfile \
	fuzzers/afl

.afl-php_fuzz-runner: .afl-php_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/afl/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/afl/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=afl \
	--file docker/benchmark-runner/Dockerfile \
	.

run-afl-php_fuzz: .afl-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/afl/php_fuzz

debug-afl-php_fuzz: .afl-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/afl/php_fuzz

test-run-afl-php_fuzz: .afl-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/afl/php_fuzz

debug-builder-afl-php_fuzz: .afl-php_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=afl \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/afl/php_fuzz

.aflplusplus-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/aflplusplus/builder.Dockerfile \
	fuzzers/aflplusplus

.aflplusplus-jerryscript_fuzz-builder-debug: .aflplusplus-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=aflplusplus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.aflplusplus-jerryscript_fuzz-builder: .aflplusplus-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=aflplusplus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.aflplusplus-jerryscript_fuzz-intermediate-runner: .aflplusplus-jerryscript_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/aflplusplus/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/aflplusplus/jerryscript_fuzz-intermediate \
	--file fuzzers/aflplusplus/runner.Dockerfile \
	fuzzers/aflplusplus

.aflplusplus-jerryscript_fuzz-runner: .aflplusplus-jerryscript_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/aflplusplus/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/aflplusplus/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=aflplusplus \
	--file docker/benchmark-runner/Dockerfile \
	.

run-aflplusplus-jerryscript_fuzz: .aflplusplus-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/aflplusplus/jerryscript_fuzz

debug-aflplusplus-jerryscript_fuzz: .aflplusplus-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/aflplusplus/jerryscript_fuzz

test-run-aflplusplus-jerryscript_fuzz: .aflplusplus-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/aflplusplus/jerryscript_fuzz

debug-builder-aflplusplus-jerryscript_fuzz: .aflplusplus-jerryscript_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/aflplusplus/jerryscript_fuzz

.aflplusplus-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/aflplusplus/builder.Dockerfile \
	fuzzers/aflplusplus

.aflplusplus-mruby_fuzz-builder-debug: .aflplusplus-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=aflplusplus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.aflplusplus-mruby_fuzz-builder: .aflplusplus-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=aflplusplus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.aflplusplus-mruby_fuzz-intermediate-runner: .aflplusplus-mruby_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/aflplusplus/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/aflplusplus/mruby_fuzz-intermediate \
	--file fuzzers/aflplusplus/runner.Dockerfile \
	fuzzers/aflplusplus

.aflplusplus-mruby_fuzz-runner: .aflplusplus-mruby_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/aflplusplus/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/aflplusplus/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=aflplusplus \
	--file docker/benchmark-runner/Dockerfile \
	.

run-aflplusplus-mruby_fuzz: .aflplusplus-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/aflplusplus/mruby_fuzz

debug-aflplusplus-mruby_fuzz: .aflplusplus-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/aflplusplus/mruby_fuzz

test-run-aflplusplus-mruby_fuzz: .aflplusplus-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/aflplusplus/mruby_fuzz

debug-builder-aflplusplus-mruby_fuzz: .aflplusplus-mruby_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/aflplusplus/mruby_fuzz

.aflplusplus-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/aflplusplus/builder.Dockerfile \
	fuzzers/aflplusplus

.aflplusplus-php_fuzz-builder-debug: .aflplusplus-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/php_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=aflplusplus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/aflplusplus/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.aflplusplus-php_fuzz-builder: .aflplusplus-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/aflplusplus/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/aflplusplus/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=aflplusplus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/aflplusplus/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.aflplusplus-php_fuzz-intermediate-runner: .aflplusplus-php_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/aflplusplus/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/aflplusplus/php_fuzz-intermediate \
	--file fuzzers/aflplusplus/runner.Dockerfile \
	fuzzers/aflplusplus

.aflplusplus-php_fuzz-runner: .aflplusplus-php_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/aflplusplus/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/aflplusplus/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=aflplusplus \
	--file docker/benchmark-runner/Dockerfile \
	.

run-aflplusplus-php_fuzz: .aflplusplus-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/aflplusplus/php_fuzz

debug-aflplusplus-php_fuzz: .aflplusplus-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/aflplusplus/php_fuzz

test-run-aflplusplus-php_fuzz: .aflplusplus-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/aflplusplus/php_fuzz

debug-builder-aflplusplus-php_fuzz: .aflplusplus-php_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=aflplusplus \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/aflplusplus/php_fuzz

.custom_cov-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/custom_cov/builder.Dockerfile \
	fuzzers/custom_cov

.custom_cov-jerryscript_fuzz-builder-debug: .custom_cov-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=custom_cov \
	--build-arg parent_image=gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.custom_cov-jerryscript_fuzz-builder: .custom_cov-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=custom_cov \
	--build-arg parent_image=gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.custom_cov-jerryscript_fuzz-intermediate-runner: .custom_cov-jerryscript_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/custom_cov/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/custom_cov/jerryscript_fuzz-intermediate \
	--file fuzzers/custom_cov/runner.Dockerfile \
	fuzzers/custom_cov

.custom_cov-jerryscript_fuzz-runner: .custom_cov-jerryscript_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/custom_cov/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/custom_cov/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=custom_cov \
	--file docker/benchmark-runner/Dockerfile \
	.

run-custom_cov-jerryscript_fuzz: .custom_cov-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/custom_cov/jerryscript_fuzz

debug-custom_cov-jerryscript_fuzz: .custom_cov-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/custom_cov/jerryscript_fuzz

test-run-custom_cov-jerryscript_fuzz: .custom_cov-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/custom_cov/jerryscript_fuzz

debug-builder-custom_cov-jerryscript_fuzz: .custom_cov-jerryscript_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/custom_cov/jerryscript_fuzz

.custom_cov-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/custom_cov/builder.Dockerfile \
	fuzzers/custom_cov

.custom_cov-mruby_fuzz-builder-debug: .custom_cov-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=custom_cov \
	--build-arg parent_image=gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.custom_cov-mruby_fuzz-builder: .custom_cov-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=custom_cov \
	--build-arg parent_image=gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.custom_cov-mruby_fuzz-intermediate-runner: .custom_cov-mruby_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/custom_cov/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/custom_cov/mruby_fuzz-intermediate \
	--file fuzzers/custom_cov/runner.Dockerfile \
	fuzzers/custom_cov

.custom_cov-mruby_fuzz-runner: .custom_cov-mruby_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/custom_cov/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/custom_cov/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=custom_cov \
	--file docker/benchmark-runner/Dockerfile \
	.

run-custom_cov-mruby_fuzz: .custom_cov-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/custom_cov/mruby_fuzz

debug-custom_cov-mruby_fuzz: .custom_cov-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/custom_cov/mruby_fuzz

test-run-custom_cov-mruby_fuzz: .custom_cov-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/custom_cov/mruby_fuzz

debug-builder-custom_cov-mruby_fuzz: .custom_cov-mruby_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/custom_cov/mruby_fuzz

.custom_cov-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/custom_cov/builder.Dockerfile \
	fuzzers/custom_cov

.custom_cov-php_fuzz-builder-debug: .custom_cov-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/php_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=custom_cov \
	--build-arg parent_image=gcr.io/fuzzbench/builders/custom_cov/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.custom_cov-php_fuzz-builder: .custom_cov-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/custom_cov/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/custom_cov/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=custom_cov \
	--build-arg parent_image=gcr.io/fuzzbench/builders/custom_cov/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.custom_cov-php_fuzz-intermediate-runner: .custom_cov-php_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/custom_cov/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/custom_cov/php_fuzz-intermediate \
	--file fuzzers/custom_cov/runner.Dockerfile \
	fuzzers/custom_cov

.custom_cov-php_fuzz-runner: .custom_cov-php_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/custom_cov/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/custom_cov/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=custom_cov \
	--file docker/benchmark-runner/Dockerfile \
	.

run-custom_cov-php_fuzz: .custom_cov-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/custom_cov/php_fuzz

debug-custom_cov-php_fuzz: .custom_cov-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/custom_cov/php_fuzz

test-run-custom_cov-php_fuzz: .custom_cov-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/custom_cov/php_fuzz

debug-builder-custom_cov-php_fuzz: .custom_cov-php_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=custom_cov \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/custom_cov/php_fuzz

.gramatron-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/gramatron/builder.Dockerfile \
	fuzzers/gramatron

.gramatron-jerryscript_fuzz-builder-debug: .gramatron-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=gramatron \
	--build-arg parent_image=gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.gramatron-jerryscript_fuzz-builder: .gramatron-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=gramatron \
	--build-arg parent_image=gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.gramatron-jerryscript_fuzz-intermediate-runner: .gramatron-jerryscript_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/gramatron/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/gramatron/jerryscript_fuzz-intermediate \
	--file fuzzers/gramatron/runner.Dockerfile \
	fuzzers/gramatron

.gramatron-jerryscript_fuzz-runner: .gramatron-jerryscript_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/gramatron/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/gramatron/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=gramatron \
	--file docker/benchmark-runner/Dockerfile \
	.

run-gramatron-jerryscript_fuzz: .gramatron-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/gramatron/jerryscript_fuzz

debug-gramatron-jerryscript_fuzz: .gramatron-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/gramatron/jerryscript_fuzz

test-run-gramatron-jerryscript_fuzz: .gramatron-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/gramatron/jerryscript_fuzz

debug-builder-gramatron-jerryscript_fuzz: .gramatron-jerryscript_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/gramatron/jerryscript_fuzz

.gramatron-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/gramatron/builder.Dockerfile \
	fuzzers/gramatron

.gramatron-mruby_fuzz-builder-debug: .gramatron-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/mruby_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=gramatron \
	--build-arg parent_image=gcr.io/fuzzbench/builders/gramatron/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.gramatron-mruby_fuzz-builder: .gramatron-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=gramatron \
	--build-arg parent_image=gcr.io/fuzzbench/builders/gramatron/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.gramatron-mruby_fuzz-intermediate-runner: .gramatron-mruby_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/gramatron/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/gramatron/mruby_fuzz-intermediate \
	--file fuzzers/gramatron/runner.Dockerfile \
	fuzzers/gramatron

.gramatron-mruby_fuzz-runner: .gramatron-mruby_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/gramatron/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/gramatron/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=gramatron \
	--file docker/benchmark-runner/Dockerfile \
	.

run-gramatron-mruby_fuzz: .gramatron-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/gramatron/mruby_fuzz

debug-gramatron-mruby_fuzz: .gramatron-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/gramatron/mruby_fuzz

test-run-gramatron-mruby_fuzz: .gramatron-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/gramatron/mruby_fuzz

debug-builder-gramatron-mruby_fuzz: .gramatron-mruby_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/gramatron/mruby_fuzz

.gramatron-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/gramatron/builder.Dockerfile \
	fuzzers/gramatron

.gramatron-php_fuzz-builder-debug: .gramatron-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/php_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=gramatron \
	--build-arg parent_image=gcr.io/fuzzbench/builders/gramatron/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.gramatron-php_fuzz-builder: .gramatron-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/gramatron/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/gramatron/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=gramatron \
	--build-arg parent_image=gcr.io/fuzzbench/builders/gramatron/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.gramatron-php_fuzz-intermediate-runner: .gramatron-php_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/gramatron/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/gramatron/php_fuzz-intermediate \
	--file fuzzers/gramatron/runner.Dockerfile \
	fuzzers/gramatron

.gramatron-php_fuzz-runner: .gramatron-php_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/gramatron/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/gramatron/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=gramatron \
	--file docker/benchmark-runner/Dockerfile \
	.

run-gramatron-php_fuzz: .gramatron-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/gramatron/php_fuzz

debug-gramatron-php_fuzz: .gramatron-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/gramatron/php_fuzz

test-run-gramatron-php_fuzz: .gramatron-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/gramatron/php_fuzz

debug-builder-gramatron-php_fuzz: .gramatron-php_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=gramatron \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/gramatron/php_fuzz

.nautilus-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/nautilus/builder.Dockerfile \
	fuzzers/nautilus

.nautilus-jerryscript_fuzz-builder-debug: .nautilus-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=nautilus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.nautilus-jerryscript_fuzz-builder: .nautilus-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=nautilus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.nautilus-jerryscript_fuzz-intermediate-runner: .nautilus-jerryscript_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/nautilus/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/nautilus/jerryscript_fuzz-intermediate \
	--file fuzzers/nautilus/runner.Dockerfile \
	fuzzers/nautilus

.nautilus-jerryscript_fuzz-runner: .nautilus-jerryscript_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/nautilus/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/nautilus/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=nautilus \
	--file docker/benchmark-runner/Dockerfile \
	.

run-nautilus-jerryscript_fuzz: .nautilus-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/nautilus/jerryscript_fuzz

debug-nautilus-jerryscript_fuzz: .nautilus-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/nautilus/jerryscript_fuzz

test-run-nautilus-jerryscript_fuzz: .nautilus-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/nautilus/jerryscript_fuzz

debug-builder-nautilus-jerryscript_fuzz: .nautilus-jerryscript_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/nautilus/jerryscript_fuzz

.nautilus-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/nautilus/builder.Dockerfile \
	fuzzers/nautilus

.nautilus-mruby_fuzz-builder-debug: .nautilus-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/mruby_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=nautilus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/nautilus/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.nautilus-mruby_fuzz-builder: .nautilus-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=nautilus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/nautilus/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.nautilus-mruby_fuzz-intermediate-runner: .nautilus-mruby_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/nautilus/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/nautilus/mruby_fuzz-intermediate \
	--file fuzzers/nautilus/runner.Dockerfile \
	fuzzers/nautilus

.nautilus-mruby_fuzz-runner: .nautilus-mruby_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/nautilus/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/nautilus/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=nautilus \
	--file docker/benchmark-runner/Dockerfile \
	.

run-nautilus-mruby_fuzz: .nautilus-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/nautilus/mruby_fuzz

debug-nautilus-mruby_fuzz: .nautilus-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/nautilus/mruby_fuzz

test-run-nautilus-mruby_fuzz: .nautilus-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/nautilus/mruby_fuzz

debug-builder-nautilus-mruby_fuzz: .nautilus-mruby_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/nautilus/mruby_fuzz

.nautilus-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/nautilus/builder.Dockerfile \
	fuzzers/nautilus

.nautilus-php_fuzz-builder-debug: .nautilus-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/php_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=nautilus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/nautilus/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.nautilus-php_fuzz-builder: .nautilus-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/nautilus/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/nautilus/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=nautilus \
	--build-arg parent_image=gcr.io/fuzzbench/builders/nautilus/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.nautilus-php_fuzz-intermediate-runner: .nautilus-php_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/nautilus/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/nautilus/php_fuzz-intermediate \
	--file fuzzers/nautilus/runner.Dockerfile \
	fuzzers/nautilus

.nautilus-php_fuzz-runner: .nautilus-php_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/nautilus/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/nautilus/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=nautilus \
	--file docker/benchmark-runner/Dockerfile \
	.

run-nautilus-php_fuzz: .nautilus-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/nautilus/php_fuzz

debug-nautilus-php_fuzz: .nautilus-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/nautilus/php_fuzz

test-run-nautilus-php_fuzz: .nautilus-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/nautilus/php_fuzz

debug-builder-nautilus-php_fuzz: .nautilus-php_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=nautilus \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/nautilus/php_fuzz

.thesis_php-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/thesis_php/builder.Dockerfile \
	fuzzers/thesis_php

.thesis_php-jerryscript_fuzz-builder-debug: .thesis_php-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_php \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_php-jerryscript_fuzz-builder: .thesis_php-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_php \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_php-jerryscript_fuzz-intermediate-runner: .thesis_php-jerryscript_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_php/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_php/jerryscript_fuzz-intermediate \
	--file fuzzers/thesis_php/runner.Dockerfile \
	fuzzers/thesis_php

.thesis_php-jerryscript_fuzz-runner: .thesis_php-jerryscript_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_php/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_php/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_php \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_php-jerryscript_fuzz: .thesis_php-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_php/jerryscript_fuzz

debug-thesis_php-jerryscript_fuzz: .thesis_php-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_php/jerryscript_fuzz

test-run-thesis_php-jerryscript_fuzz: .thesis_php-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_php/jerryscript_fuzz

debug-builder-thesis_php-jerryscript_fuzz: .thesis_php-jerryscript_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_php/jerryscript_fuzz

.thesis_php-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/thesis_php/builder.Dockerfile \
	fuzzers/thesis_php

.thesis_php-mruby_fuzz-builder-debug: .thesis_php-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_php \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_php-mruby_fuzz-builder: .thesis_php-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_php \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_php-mruby_fuzz-intermediate-runner: .thesis_php-mruby_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_php/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_php/mruby_fuzz-intermediate \
	--file fuzzers/thesis_php/runner.Dockerfile \
	fuzzers/thesis_php

.thesis_php-mruby_fuzz-runner: .thesis_php-mruby_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_php/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_php/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_php \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_php-mruby_fuzz: .thesis_php-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_php/mruby_fuzz

debug-thesis_php-mruby_fuzz: .thesis_php-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_php/mruby_fuzz

test-run-thesis_php-mruby_fuzz: .thesis_php-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_php/mruby_fuzz

debug-builder-thesis_php-mruby_fuzz: .thesis_php-mruby_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_php/mruby_fuzz

.thesis_php-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/thesis_php/builder.Dockerfile \
	fuzzers/thesis_php

.thesis_php-php_fuzz-builder-debug: .thesis_php-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/php_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_php \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_php/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_php-php_fuzz-builder: .thesis_php-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_php/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_php/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_php \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_php/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_php-php_fuzz-intermediate-runner: .thesis_php-php_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_php/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_php/php_fuzz-intermediate \
	--file fuzzers/thesis_php/runner.Dockerfile \
	fuzzers/thesis_php

.thesis_php-php_fuzz-runner: .thesis_php-php_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_php/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_php/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_php \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_php-php_fuzz: .thesis_php-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_php/php_fuzz

debug-thesis_php-php_fuzz: .thesis_php-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_php/php_fuzz

test-run-thesis_php-php_fuzz: .thesis_php-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_php/php_fuzz

debug-builder-thesis_php-php_fuzz: .thesis_php-php_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_php \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_php/php_fuzz

.thesis_js-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/thesis_js/builder.Dockerfile \
	fuzzers/thesis_js

.thesis_js-jerryscript_fuzz-builder-debug: .thesis_js-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_js \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_js-jerryscript_fuzz-builder: .thesis_js-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_js \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_js-jerryscript_fuzz-intermediate-runner: .thesis_js-jerryscript_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_js/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_js/jerryscript_fuzz-intermediate \
	--file fuzzers/thesis_js/runner.Dockerfile \
	fuzzers/thesis_js

.thesis_js-jerryscript_fuzz-runner: .thesis_js-jerryscript_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_js/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_js/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_js \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_js-jerryscript_fuzz: .thesis_js-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_js/jerryscript_fuzz

debug-thesis_js-jerryscript_fuzz: .thesis_js-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_js/jerryscript_fuzz

test-run-thesis_js-jerryscript_fuzz: .thesis_js-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_js/jerryscript_fuzz

debug-builder-thesis_js-jerryscript_fuzz: .thesis_js-jerryscript_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_js/jerryscript_fuzz

.thesis_js-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/thesis_js/builder.Dockerfile \
	fuzzers/thesis_js

.thesis_js-mruby_fuzz-builder-debug: .thesis_js-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_js \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_js-mruby_fuzz-builder: .thesis_js-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_js \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_js-mruby_fuzz-intermediate-runner: .thesis_js-mruby_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_js/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_js/mruby_fuzz-intermediate \
	--file fuzzers/thesis_js/runner.Dockerfile \
	fuzzers/thesis_js

.thesis_js-mruby_fuzz-runner: .thesis_js-mruby_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_js/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_js/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_js \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_js-mruby_fuzz: .thesis_js-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_js/mruby_fuzz

debug-thesis_js-mruby_fuzz: .thesis_js-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_js/mruby_fuzz

test-run-thesis_js-mruby_fuzz: .thesis_js-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_js/mruby_fuzz

debug-builder-thesis_js-mruby_fuzz: .thesis_js-mruby_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_js/mruby_fuzz

.thesis_js-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/thesis_js/builder.Dockerfile \
	fuzzers/thesis_js

.thesis_js-php_fuzz-builder-debug: .thesis_js-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/php_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_js \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_js/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_js-php_fuzz-builder: .thesis_js-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_js/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_js/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_js \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_js/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_js-php_fuzz-intermediate-runner: .thesis_js-php_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_js/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_js/php_fuzz-intermediate \
	--file fuzzers/thesis_js/runner.Dockerfile \
	fuzzers/thesis_js

.thesis_js-php_fuzz-runner: .thesis_js-php_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_js/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_js/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_js \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_js-php_fuzz: .thesis_js-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_js/php_fuzz

debug-thesis_js-php_fuzz: .thesis_js-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_js/php_fuzz

test-run-thesis_js-php_fuzz: .thesis_js-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_js/php_fuzz

debug-builder-thesis_js-php_fuzz: .thesis_js-php_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_js \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_js/php_fuzz

.thesis_ruby-jerryscript_fuzz-builder-intermediate: .jerryscript_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/jerryscript_fuzz \
	--file fuzzers/thesis_ruby/builder.Dockerfile \
	fuzzers/thesis_ruby

.thesis_ruby-jerryscript_fuzz-builder-debug: .thesis_ruby-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_ruby-jerryscript_fuzz-builder: .thesis_ruby-jerryscript_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_ruby-jerryscript_fuzz-intermediate-runner: .thesis_ruby-jerryscript_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_ruby/jerryscript_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_ruby/jerryscript_fuzz-intermediate \
	--file fuzzers/thesis_ruby/runner.Dockerfile \
	fuzzers/thesis_ruby

.thesis_ruby-jerryscript_fuzz-runner: .thesis_ruby-jerryscript_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_ruby/jerryscript_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_ruby/jerryscript_fuzz \
	--build-arg benchmark=jerryscript_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_ruby-jerryscript_fuzz: .thesis_ruby-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_ruby/jerryscript_fuzz

debug-thesis_ruby-jerryscript_fuzz: .thesis_ruby-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_ruby/jerryscript_fuzz

test-run-thesis_ruby-jerryscript_fuzz: .thesis_ruby-jerryscript_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_ruby/jerryscript_fuzz

debug-builder-thesis_ruby-jerryscript_fuzz: .thesis_ruby-jerryscript_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=jerryscript_fuzz \
	-e FUZZ_TARGET=$(jerryscript_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_ruby/jerryscript_fuzz

.thesis_ruby-mruby_fuzz-builder-intermediate: .mruby_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/mruby_fuzz \
	--file fuzzers/thesis_ruby/builder.Dockerfile \
	fuzzers/thesis_ruby

.thesis_ruby-mruby_fuzz-builder-debug: .thesis_ruby-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_ruby-mruby_fuzz-builder: .thesis_ruby-mruby_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_ruby-mruby_fuzz-intermediate-runner: .thesis_ruby-mruby_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_ruby/mruby_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_ruby/mruby_fuzz-intermediate \
	--file fuzzers/thesis_ruby/runner.Dockerfile \
	fuzzers/thesis_ruby

.thesis_ruby-mruby_fuzz-runner: .thesis_ruby-mruby_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_ruby/mruby_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_ruby/mruby_fuzz \
	--build-arg benchmark=mruby_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_ruby-mruby_fuzz: .thesis_ruby-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_ruby/mruby_fuzz

debug-thesis_ruby-mruby_fuzz: .thesis_ruby-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_ruby/mruby_fuzz

test-run-thesis_ruby-mruby_fuzz: .thesis_ruby-mruby_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_ruby/mruby_fuzz

debug-builder-thesis_ruby-mruby_fuzz: .thesis_ruby-mruby_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=mruby_fuzz \
	-e FUZZ_TARGET=$(mruby_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_ruby/mruby_fuzz

.thesis_ruby-php_fuzz-builder-intermediate: .php_fuzz-project-builder
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz-intermediate \
	--build-arg parent_image=gcr.io/fuzzbench/builders/benchmark/php_fuzz \
	--file fuzzers/thesis_ruby/builder.Dockerfile \
	fuzzers/thesis_ruby

.thesis_ruby-php_fuzz-builder-debug: .thesis_ruby-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz \
	--build-arg debug_builder=1 \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_ruby-php_fuzz-builder: .thesis_ruby-php_fuzz-builder-intermediate base-image
	docker build \
	--tag gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--build-arg parent_image=gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz-intermediate \
	--file docker/benchmark-builder/Dockerfile \
	.

.thesis_ruby-php_fuzz-intermediate-runner: .thesis_ruby-php_fuzz-builder base-image
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_ruby/php_fuzz-intermediate \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_ruby/php_fuzz-intermediate \
	--file fuzzers/thesis_ruby/runner.Dockerfile \
	fuzzers/thesis_ruby

.thesis_ruby-php_fuzz-runner: .thesis_ruby-php_fuzz-intermediate-runner
	docker build \
	--tag gcr.io/fuzzbench/runners/thesis_ruby/php_fuzz \
	--build-arg BUILDKIT_INLINE_CACHE=1 \
	--cache-from gcr.io/fuzzbench/runners/thesis_ruby/php_fuzz \
	--build-arg benchmark=php_fuzz \
	--build-arg fuzzer=thesis_ruby \
	--file docker/benchmark-runner/Dockerfile \
	.

run-thesis_ruby-php_fuzz: .thesis_ruby-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-it gcr.io/fuzzbench/runners/thesis_ruby/php_fuzz

debug-thesis_ruby-php_fuzz: .thesis_ruby-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/runners/thesis_ruby/php_fuzz

test-run-thesis_ruby-php_fuzz: .thesis_ruby-php_fuzz-runner
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e MAX_TOTAL_TIME=20 \
	-e SNAPSHOT_PERIOD=10 \
	gcr.io/fuzzbench/runners/thesis_ruby/php_fuzz

debug-builder-thesis_ruby-php_fuzz: .thesis_ruby-php_fuzz-builder-debug
	docker run \
	--cpus=1 \
	--shm-size=2g \
	--cap-add SYS_NICE \
	--cap-add SYS_PTRACE \
	-e FUZZ_OUTSIDE_EXPERIMENT=1 \
	-e FORCE_LOCAL=1 \
	-e TRIAL_ID=1 \
	-e FUZZER=thesis_ruby \
	-e BENCHMARK=php_fuzz \
	-e FUZZ_TARGET=$(php_fuzz-fuzz-target) \
	-e DEBUG_BUILDER=1 \
	--entrypoint "/bin/bash" \
	-it gcr.io/fuzzbench/builders/thesis_ruby/php_fuzz

build-afl-jerryscript_fuzz: .afl-jerryscript_fuzz-runner
build-afl-mruby_fuzz: .afl-mruby_fuzz-runner
build-afl-php_fuzz: .afl-php_fuzz-runner

build-aflplusplus-jerryscript_fuzz: .aflplusplus-jerryscript_fuzz-runner
build-aflplusplus-mruby_fuzz: .aflplusplus-mruby_fuzz-runner
build-aflplusplus-php_fuzz: .aflplusplus-php_fuzz-runner

build-custom_cov-jerryscript_fuzz: .custom_cov-jerryscript_fuzz-runner
build-custom_cov-mruby_fuzz: .custom_cov-mruby_fuzz-runner
build-custom_cov-php_fuzz: .custom_cov-php_fuzz-runner

build-gramatron-jerryscript_fuzz: .gramatron-jerryscript_fuzz-runner
build-gramatron-mruby_fuzz: .gramatron-mruby_fuzz-runner
build-gramatron-php_fuzz: .gramatron-php_fuzz-runner

build-nautilus-jerryscript_fuzz: .nautilus-jerryscript_fuzz-runner
build-nautilus-mruby_fuzz: .nautilus-mruby_fuzz-runner
build-nautilus-php_fuzz: .nautilus-php_fuzz-runner

build-thesis_php-jerryscript_fuzz: .thesis_php-jerryscript_fuzz-runner
build-thesis_php-mruby_fuzz: .thesis_php-mruby_fuzz-runner
build-thesis_php-php_fuzz: .thesis_php-php_fuzz-runner

build-thesis_js-jerryscript_fuzz: .thesis_js-jerryscript_fuzz-runner
build-thesis_js-mruby_fuzz: .thesis_js-mruby_fuzz-runner
build-thesis_js-php_fuzz: .thesis_js-php_fuzz-runner

build-thesis_ruby-jerryscript_fuzz: .thesis_ruby-jerryscript_fuzz-runner
build-thesis_ruby-mruby_fuzz: .thesis_ruby-mruby_fuzz-runner
build-thesis_ruby-php_fuzz: .thesis_ruby-php_fuzz-runner

build-coverage-jerryscript_fuzz: .coverage-jerryscript_fuzz-builder
build-coverage-mruby_fuzz: .coverage-mruby_fuzz-builder
build-coverage-php_fuzz: .coverage-php_fuzz-builder

build-afl-all: build-afl-jerryscript_fuzz build-afl-mruby_fuzz build-afl-php_fuzz
test-run-afl-all: test-run-afl-jerryscript_fuzz test-run-afl-mruby_fuzz test-run-afl-php_fuzz
build-aflplusplus-all: build-aflplusplus-jerryscript_fuzz build-aflplusplus-mruby_fuzz build-aflplusplus-php_fuzz
test-run-aflplusplus-all: test-run-aflplusplus-jerryscript_fuzz test-run-aflplusplus-mruby_fuzz test-run-aflplusplus-php_fuzz
build-custom_cov-all: build-custom_cov-jerryscript_fuzz build-custom_cov-mruby_fuzz build-custom_cov-php_fuzz
test-run-custom_cov-all: test-run-custom_cov-jerryscript_fuzz test-run-custom_cov-mruby_fuzz test-run-custom_cov-php_fuzz
build-gramatron-all: build-gramatron-jerryscript_fuzz build-gramatron-mruby_fuzz build-gramatron-php_fuzz
test-run-gramatron-all: test-run-gramatron-jerryscript_fuzz test-run-gramatron-mruby_fuzz test-run-gramatron-php_fuzz
build-nautilus-all: build-nautilus-jerryscript_fuzz build-nautilus-mruby_fuzz build-nautilus-php_fuzz
test-run-nautilus-all: test-run-nautilus-jerryscript_fuzz test-run-nautilus-mruby_fuzz test-run-nautilus-php_fuzz
build-thesis_php-all: build-thesis_php-jerryscript_fuzz build-thesis_php-mruby_fuzz build-thesis_php-php_fuzz
test-run-thesis_php-all: test-run-thesis_php-jerryscript_fuzz test-run-thesis_php-mruby_fuzz test-run-thesis_php-php_fuzz
build-thesis_js-all: build-thesis_js-jerryscript_fuzz build-thesis_js-mruby_fuzz build-thesis_js-php_fuzz
test-run-thesis_js-all: test-run-thesis_js-jerryscript_fuzz test-run-thesis_js-mruby_fuzz test-run-thesis_js-php_fuzz
build-thesis_ruby-all: build-thesis_ruby-jerryscript_fuzz build-thesis_ruby-mruby_fuzz build-thesis_ruby-php_fuzz
test-run-thesis_ruby-all: test-run-thesis_ruby-jerryscript_fuzz test-run-thesis_ruby-mruby_fuzz test-run-thesis_ruby-php_fuzz
build-coverage-all: build-coverage-jerryscript_fuzz build-coverage-mruby_fuzz build-coverage-php_fuzz
test-run-coverage-all: test-run-coverage-jerryscript_fuzz test-run-coverage-mruby_fuzz test-run-coverage-php_fuzz
build-all: build-afl-all build-aflplusplus-all build-custom_cov-all build-gramatron-all build-nautilus-all build-thesis_php-all build-thesis_js-all build-thesis_ruby-all build-coverage-all