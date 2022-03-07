.PHONY: cs web

all: cs

cs:
	cargo run
	
web:
	cd web/ && trunk serve