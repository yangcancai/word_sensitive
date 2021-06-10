#!/bin/bash

check(){
	cargo run
	valgrind --tool=memcheck --leak-check=full ./target/debug/leak
}
help(){
	echo "sh tool.sh check -- check leak"
}
case $1 in
	check) check;;
	*) help;;
esac