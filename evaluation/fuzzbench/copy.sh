rm -rf ./fuzzers/thesis_php/libafl-fuzzer
rm -rf ./fuzzers/thesis_php/php
rm -rf ./fuzzers/thesis_php/thesis_derive
rm -rf ./fuzzers/thesis_php/thesis

cp -r ../../libafl-fuzzer ./fuzzers/thesis_php/libafl-fuzzer
cp -r ../../unparser-php ./fuzzers/thesis_php/php
cp -r ../../thesis_derive ./fuzzers/thesis_php/thesis_derive
cp -r ../../thesis ./fuzzers/thesis_php/thesis

rm -rf ./fuzzers/thesis_js/libafl-fuzzer
rm -rf ./fuzzers/thesis_js/js
rm -rf ./fuzzers/thesis_js/thesis_derive
rm -rf ./fuzzers/thesis_js/thesis

cp -r ../../libafl-fuzzer ./fuzzers/thesis_js/libafl-fuzzer
cp -r ../../unparser-js ./fuzzers/thesis_js/js
cp -r ../../thesis_derive ./fuzzers/thesis_js/thesis_derive
cp -r ../../thesis ./fuzzers/thesis_js/thesis

rm -rf ./fuzzers/thesis_ruby/libafl-fuzzer
rm -rf ./fuzzers/thesis_ruby/ruby
rm -rf ./fuzzers/thesis_ruby/thesis_derive
rm -rf ./fuzzers/thesis_ruby/thesis

cp -r ../../libafl-fuzzer ./fuzzers/thesis_ruby/libafl-fuzzer
cp -r ../../unparser-ruby ./fuzzers/thesis_ruby/ruby
cp -r ../../thesis_derive ./fuzzers/thesis_ruby/thesis_derive
cp -r ../../thesis ./fuzzers/thesis_ruby/thesis
