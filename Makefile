.PHONY: s c db-init

all: s

mysql-init:
	mysql -h localhost -uroot -p cuckooemm entity/sql/mysql.sql
s:
	cargo run
c:
	cargo check