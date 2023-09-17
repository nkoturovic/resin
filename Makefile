prog :=resin

debug ?=

$(info debug is $(debug))

ifdef release_type
  release_type :=
  target :=release
  extension :=release
else
  release_type :=--debug
  target :=debug
  extension :=
endif

build:
	cargo build $(release_type)

run:
	cargo run -- $(release_type)

migrate-init:
	ormlite migrate initial

migrate-up:
	ormlite up


# install:
# 	cp target/$(target)/$(prog) ~/.local/bin/$(prog)-$(extension)

# all: build install
 
help:
	@echo "usage: make $(prog) [release=1]"
