alias w := watch
alias b := build
alias cfg := configure



configure:
	just install-dependencies

watch target:
	just watch-{{target}}


build target:
	just build-{{target}}
