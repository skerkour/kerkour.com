.PHONY: all
all: build

.PHONY: build
build:
	hugo

.PHONY: dev
dev:
	hugo server --buildDrafts

.PHONY: gzip
gzip:
	gzip -k -9 -r -f public
