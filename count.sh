echo -e "\x1b[36m----------------all steps written by rust----------------\x1b[0m"
echo -e "\x1b[31mlines  words bytes filename\x1b[0m"
wc  src/*.rs src/ir/*.rs src/lex/*.rs src/parse/*.rs src/manager/*.rs src/token/*.rs
