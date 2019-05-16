echo '----------------all steps written by golang----------------'
wc -l golang/*.go golang/codegen/*.go golang/asm/*.go golang/parse/*.go golang/lex/*.go golang/token/*.go golang/pkg/*.go
echo ''
echo ''
echo ''
echo '----------------all steps written by rust----------------'
wc -l rust/src/*.rs rust/src/lex/*.rs rust/src/elf/*.rs
