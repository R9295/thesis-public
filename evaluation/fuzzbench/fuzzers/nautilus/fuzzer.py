# Copyright 2020 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
"""Integration code for a LibAFL-based fuzzer."""

import os
import shutil
import subprocess

from fuzzers import utils


def prepare_fuzz_environment(input_corpus):
    """Prepare to fuzz with a LibAFL-based fuzzer."""
    os.environ['ASAN_OPTIONS'] = 'abort_on_error=1:detect_leaks=0:'\
                                 'malloc_context_size=0:symbolize=0:'\
                                 'allocator_may_return_null=1:'\
                                 'detect_odr_violation=0:handle_segv=0:'\
                                 'handle_sigbus=0:handle_abort=0:'\
                                 'handle_sigfpe=0:handle_sigill=0'
    os.environ['UBSAN_OPTIONS'] =  'abort_on_error=1:'\
                                   'allocator_release_to_os_interval_ms=500:'\
                                   'handle_abort=0:handle_segv=0:'\
                                   'handle_sigbus=0:handle_sigfpe=0:'\
                                   'handle_sigill=0:print_stacktrace=0:'\
                                   'symbolize=0:symbolize_inline_frames=0'
    # Create at least one non-empty seed to start.
    utils.create_seed_file_for_empty_corpus(input_corpus)


def build():  # pylint: disable=too-many-branches,too-many-statements
    src = os.getenv('SRC')
    work = os.getenv('WORK')
    os.environ['CC'] = '/afl/afl-clang-fast'
    os.environ['CXX'] = '/afl/afl-clang-fast++'
    
    build_directory = os.environ['OUT']
    fuzzer = '/nautilus/target/release/fuzzer'
    config = '/nautilus/config.ron'

    """Build benchmark."""
    benchmark_name = os.environ['BENCHMARK'].lower()
    if 'php' in benchmark_name:
        copy_file = '/nautilus/grammars/php_custom.py'
    elif 'ruby' in benchmark_name:
        copy_file = '/nautilus/grammars/ruby_custom.py'
    elif 'jerryscript' in benchmark_name or 'javascript' in benchmark_name:
        copy_file = '/nautilus/grammars/javascript_new.py'
    elif 'lua' in benchmark_name:
        copy_file = '/nautilus/grammars/lua.py'
    else:
        raise RuntimeError('Unsupported benchmark, unavailable grammar')
    
    dest = os.path.join(os.environ['OUT'], 'grammar.py')
    shutil.copy(copy_file, dest)

    os.environ['ASAN_OPTIONS'] = 'abort_on_error=0:allocator_may_return_null=1'
    os.environ['UBSAN_OPTIONS'] = 'abort_on_error=0'

    os.environ['FUZZER_LIB'] = '/libAFLDriver.a'

    shutil.copy(fuzzer, os.environ['OUT'])
    shutil.copy(config, os.environ['OUT'])
    os.environ['AFL_LLVM_INSTRUMENT'] = 'CLASSIC'
    with utils.restore_directory(src), utils.restore_directory(work):
        # Restore SRC to its initial state so we can build again without any
        # trouble. For some OSS-Fuzz projects, build_benchmark cannot be run
        # twice in the same directory without this.
        utils.build_benchmark()


def fuzz(input_corpus, output_corpus, target_binary):
    """Run fuzzer."""
    prepare_fuzz_environment(input_corpus)
    grammar = os.path.join(os.environ['OUT'], 'grammar.py')
    out = os.path.join(os.environ['OUT'], 'out')
    fuzzer = os.path.join(os.environ['OUT'], 'fuzzer')
    # NOTE: everything else is set by the config.ron file
    command = ['fuzzer']
    subprocess.check_call(['mkdir', 'out'], cwd=os.environ['OUT'])
    print(command)
    subprocess.check_call(command, cwd=os.environ['OUT'])
