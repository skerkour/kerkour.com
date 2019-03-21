.PHONY: all dev build push

all: build

dev:
	hugo serve -D

build:
	hugo -d dist

push:
	make
	git add .
	git commit -m "add"
	git push
