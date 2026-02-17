import 'scripts/kernel.just'
import 'scripts/misc.just'
import 'scripts/almond.just'
import 'scripts/lint.just'
import 'scripts/build.just'
import 'scripts/test.just'


alias w := watch
alias b := build
alias cfg := configure

configure:
	just install-dependencies
	just create-kernel-test-file

watch target:
	just watch-{{target}}

build target:
	just build-{{target}}
