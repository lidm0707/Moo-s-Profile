# Moo's Profile — dev / test / production build tasks
# `make help` lists targets.

DX             := dx
SITEMAP_SCRIPT := scripts/update-sitemap.mjs
SNAPSHOT_SCRIPT := scripts/fetch-content-snapshot.mjs
LLMS_SCRIPT := scripts/generate-llms.mjs
DIST_DIR       := dist/public

# Root-level files that must be served un-hashed at exactly /<name>
STATIC_FILES   := ads.txt robots.txt sitemap.xml llms.txt

.PHONY: help dev check test clean snapshot sitemap llms prepare e2e-install test-e2e lighthouse

help:
	@grep -E '^[a-z-]+:' $(MAKEFILE_LIST) | awk -F: '{print "  make " $$1}'

## dev: run dev server with hot reload
dev:
	$(DX) serve

## check: fast compile check
check:
	cargo check

## test: clippy + compile check (no test suite yet)
test: check
	cargo clippy -- -D warnings

## e2e-install: install Playwright + browsers for local (non-docker) runs
e2e-install:
	cd e2e && npm install && npx playwright install chromium

## test-e2e: run Playwright suite against ./dist/public in Docker (run `make build` first)
test-e2e:
	docker compose -f docker/docker-compose.yml up --build --abort-on-container-exit --exit-code-from e2e

## lighthouse: PageSpeed-style scores against ./dist/public (run `make build` first)
lighthouse:
	docker compose -f docker/docker-compose.lighthouse.yml up --build --abort-on-container-exit --exit-code-from lighthouse

## clean: remove previous dist output + target build artifacts
clean:
	rm -rf $(DIST_DIR)
	rm -rf target

## sitemap: scan routes + Supabase content, regenerate sitemap.xml
sitemap:
	node $(SITEMAP_SCRIPT)

## snapshot: fetch all Supabase content into assets/content_snapshot.json
snapshot:
	node $(SNAPSHOT_SCRIPT)

## llms: generate llms.txt for AI agents from the content snapshot
llms:
	node $(LLMS_SCRIPT)

## build: snapshot + sitemap scan + clean old dist + fresh release bundle + compose statics
build: clean snapshot sitemap
	$(DX) bundle --release --out-dir ./dist
	@node scripts/minify-css.mjs
	@node $(LLMS_SCRIPT)
	@for f in $(STATIC_FILES); do cp $$f $(DIST_DIR)/$$f; done
	@cp assets/profile.jpg $(DIST_DIR)/og-image.jpg
	@cp assets/content_snapshot.json $(DIST_DIR)/content_snapshot.json
	@node scripts/generate-content-meta.mjs
	@echo "Composed $(DIST_DIR): bundle + $(STATIC_FILES) + og-image.jpg + content_snapshot.json + content meta pages"

## prepare: alias of build (full production pipeline)
prepare: build
