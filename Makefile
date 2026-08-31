# The site lives in this monorepo at $(SITE)/ and is published by splitting
# that subdirectory out onto the er7-rust.github.io repository, whose own
# workflow builds it and ships it to Pages. Publishing is therefore two
# pushes, and forgetting the second one is the easy mistake this file exists
# to stop.

SITE := er7-rust.github.io

.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo 'check-trademarks  enforce the HL7 fair-use rules across the tree'
	@echo 'github-pages  push the monorepo, then publish the site (publish: same thing)'
	@echo 'site-dev    run the site locally at http://localhost:5173'
	@echo 'site-check  type-check the site, as CI does'

# Rules T1, T2, and T3 of spec/hl7-trademarks-fair-use/index.md, enforced
# across every page in the tree. Run it with the four cargo checks.
.PHONY: check-trademarks
check-trademarks:
	@bin/check-trademarks

# The guards cover the two ways this goes wrong. A dirty tree means the
# subtree split would publish something older than what is on disk, silently.
# A branch other than main means the split has no relationship to the site
# repository's main, and the push is rejected after the monorepo push has
# already happened.
#
# The remote is named github-pages (git@github.com:er7-rust/er7-rust.github.io.git),
# matching the sibling repositories' own convention rather than this one's
# former idiosyncratic "site" — `git subtree push --prefix=er7-rust.github.io
# github-pages main` is the operative line; everything else here is the
# guards and the push-origin-first step that line alone doesn't give you.
.PHONY: github-pages
github-pages:
	@test -z "$$(git status --porcelain)" || { \
		echo 'github-pages: working tree is dirty, commit first'; git status --short; exit 1; }
	@test "$$(git rev-parse --abbrev-ref HEAD)" = main || { \
		echo "github-pages: on $$(git rev-parse --abbrev-ref HEAD), not main"; exit 1; }
	git push origin main
	@git fetch -q github-pages main
	@before=$$(git rev-parse github-pages/main); \
	git subtree push --prefix=$(SITE) github-pages main; \
	git fetch -q github-pages main; \
	if [ "$$before" = "$$(git rev-parse github-pages/main)" ]; then \
		echo; echo 'Site unchanged; nothing to deploy.'; \
	else \
		echo; echo 'Pages is building. Watch it with:'; \
		echo '  gh run watch $$(gh run list -R er7-rust/$(SITE) -L1 --json databaseId --jq ".[0].databaseId") -R er7-rust/$(SITE)'; \
	fi

# Kept as an alias: every doc written before this target's rename
# (CHANGELOG.md, MAINTAINERS.md, tasks.md, CONTRIBUTING.md) says
# "make publish" verbatim, and a historical CHANGELOG entry is not
# something to go back and edit.
.PHONY: publish
publish: github-pages

.PHONY: site-dev
site-dev:
	cd $(SITE) && pnpm dev

.PHONY: site-check
site-check:
	cd $(SITE) && pnpm check
