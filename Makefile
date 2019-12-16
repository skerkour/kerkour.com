.PHONY: all dev build push wiki

BUILD_DIR := public

all: build

dev:
	zola serve --drafts

build:
	zola build

clean:
	rm -rf $(BUILD_DIR)

push:
	make
	git add .
	git commit -m "add"
	git push

wiki:
	make -C wiki build
