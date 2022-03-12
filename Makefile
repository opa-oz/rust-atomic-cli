# HELP
# This will output the help for each task
# thanks to https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.PHONY: help

mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
current_dir := $(patsubst %/,%,$(dir $(mkfile_path)))

bin_path = $(current_dir)/target/debug/atomic-cli
runtime_path = $(current_dir)/__tests__/runtime

#include .env
#export $(shell sed 's/=.*//' .env)

help: ## This help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help

# ========== Build ========== #
build: ## Build package
	cargo build

release: ## Build release version
	cargo build --release

# ========== Bin ========== #

clear:
	rm -rf "$(runtime_path)"
	mkdir -p "$(runtime_path)"

ahelp: build ## Run "help" for atomic processors
	"$(bin_path)" --help

# ========== Generate ========== #

ints: build clear ## Generate "ints" array to file
	"$(bin_path)" generate ints --file "$(runtime_path)/ints.json" --length 10 --from 5 --to 50

strings: build clear ## Generate "strings" array to file
	"$(bin_path)" generate strings --file "$(runtime_path)/strings.json" --length 10 --from 5 --to 50

objects: build clear ## Generate "strings" array to file
	"$(bin_path)" generate -p objects --file "$(runtime_path)/objects.json" --length 10
