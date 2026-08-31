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

# The remote is named github-pages (git@github.com:er7-rust/er7-rust.github.io.git),
# matching the sibling repositories' own convention rather than this one's
# former idiosyncratic "site". The guards (dirty tree, branch other than
# main) and the push-origin-first step live in the script, not here —
# see bin/make-github-pages for the reasoning.
.PHONY: github-pages
github-pages:
	@bin/make-github-pages

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
