git_init:
	git init -b main
	-git remote add origin https://github.com/denwong47/conch.git

# Internal use only.
git_init_commit:
	git add -f target/doc/index.html
	git add --all
	-pre-commit
	git add --all
	git commit -a -m "Initial Commit from template."

precommit_init:
	pre-commit install

doc:
	cargo doc --workspace --no-deps

docs_build: doc
docs_rebuild: doc

test:
	cargo test

setup: git_init precommit_init doc test git_init_commit
