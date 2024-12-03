%:
	cargo watch -c -q -s './ci/lint.sh' -x 'test --bin day$@' -x 'run -r --bin day$@'
