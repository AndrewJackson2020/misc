#!/bin/bash
set -e


build () {
	git clone https://github.com/zhaorui20005/pgbouncer-postgres.git
	cd pgbouncer-postgres
 	git submodule init
	git submodule update
	./autogen.sh
	./configure --with-ldap --with-pam
	make
	make install
}


run () {
	pgbouncer /home/pgbouncer/config/pgbouncer.ini
}


cli () {
	if [ "$1" =  "build" ]; then
		build	
	elif [ "$1" =  "run" ]; then
		run
	fi
}


cli "$@"
