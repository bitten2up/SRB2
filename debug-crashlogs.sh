#!/bin/sh

# if there is a debug binary there, assume we are in the repo
if test -f "$(pwd)/bin/lsdl2srb2.debug"; then
	SRB2DEBUGBINARY="bin/lsdl2srb2.debug"
	echo "$(git rev-parse --abbrev-ref HEAD) $(git rev-parse HEAD | cut -c -8)"
else
	if test -f $(which srb2).debug; then # attempt to use a fallback method
		SRB2DEBUGBINARY="$(which srb2).debug"
		grep "Sonic Robo Blast 2 v" $HOME/.srb2-doom/latest-log.txt
	else # couldn't find srb2
		printf "Couldn't find srb2\n"
		exit
	fi
fi
# get the backtrace
for i in $(tac $HOME/.srb2-doom/crash-log.txt | grep 'Backtrace:' -m 1 -B 9999 | tac | tail -n +2); do
	addr2line --exe=$SRB2DEBUGBINARY $(echo $i | sed "s/.*(//g" | sed "s/(.*//g")
done
