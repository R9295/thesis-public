# Copyright 2020 Google LLCG
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
    fuzzer = '/thesis/ruby/target/release/unparser-ruby'


    os.environ['ASAN_OPTIONS'] = 'abort_on_error=0:allocator_may_return_null=1'
    os.environ['UBSAN_OPTIONS'] = 'abort_on_error=0'

    os.environ['FUZZER_LIB'] = '/libAFLDriver.a'
    os.environ['AFL_LLVM_DICT2FILE'] =  f'{os.environ["OUT"]}/afl++.dict'
    os.environ['AFL_LLVM_DICT2FILE_NO_MAIN'] = '1'

    shutil.copy(fuzzer, os.environ['OUT'])
    with utils.restore_directory(src), utils.restore_directory(work):
        # Restore SRC to its initial state so we can build again without any
        # trouble. For some OSS-Fuzz projects, build_benchmark cannot be run
        # twice in the same directory without this.
        utils.build_benchmark()

def fuzz(input_corpus, output_corpus, target_binary):
    """Run fuzzer."""
    prepare_fuzz_environment(input_corpus)
    command = ['./unparser-ruby', '-S' ,'-c', '0', '-m', '200', '-o', output_corpus, '-x', 'afl++.dict', '-t', '1000', '-g', '1000', target_binary]
    print(command)
    subprocess.check_call(command, cwd=os.environ['OUT'])
