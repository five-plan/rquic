# The Rquic makefile.
#
# Important make rules:
#
# - `format` - reformat the code with cargo format

unset-override:
	@# unset first in case of any previous overrides
	@if rustup override list | grep `pwd` > /dev/null; then rustup override unset; fi

pre-format: unset-override
	@rustup component add rustfmt

format: pre-format
	@cargo fmt --all -- --check >/dev/null || \
	cargo fmt --all