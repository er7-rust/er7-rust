# The site lives in this monorepo at $(SITE)/ and is published by splitting
# that subdirectory out onto the er7-rust.github.io repository, whose own
# workflow builds it and ships it to Pages. Publishing is therefore two
# pushes, and forgetting the second one is the easy mistake this file exists
# to stop.

SITE := er7-rust.github.io

.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo 'publish     push the monorepo, then publish the site'
	@echo 'site-dev    run the site locally at http://localhost:5173'
	@echo 'site-check  type-check the site, as CI does'

# The guards cover the two ways this goes wrong. A dirty tree means the
# subtree split would publish something older than what is on disk, silently.
# A branch other than main means the split has no relationship to the site
# repository's main, and the push is rejected after the monorepo push has
# already happened.
.PHONY: publish
publish:
	@test -z "$$(git status --porcelain)" || { \
		echo 'publish: working tree is dirty, commit first'; git status --short; exit 1; }
	@test "$$(git rev-parse --abbrev-ref HEAD)" = main || { \
		echo "publish: on $$(git rev-parse --abbrev-ref HEAD), not main"; exit 1; }
	git push origin main
	git subtree push --prefix=$(SITE) site main
	@echo
	@echo 'Pages is building. Watch it with:'
	@echo '  gh run watch $$(gh run list -R er7-rust/$(SITE) -L1 --json databaseId --jq ".[0].databaseId") -R er7-rust/$(SITE)'

.PHONY: site-dev
site-dev:
	cd $(SITE) && pnpm dev

.PHONY: site-check
site-check:
	cd $(SITE) && pnpm check
