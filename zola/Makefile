.PHONY: all dev build push

BUILD_DIR := public

all: build

dev:
	zola serve

build:
	zola build

clean:
	rm -rf $(BUILD_DIR)

push:
	make
	git add .
	git commit -m "add"
	git push
