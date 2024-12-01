all:
	cargo watch -c -q -s './ci/lint.sh' -x 'run -r'
