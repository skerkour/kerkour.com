.PHONY: all dev build push

all: build

dev:
	hugo serve -D --themesDir hugothemes

build:
	hugo -d dist --themesDir hugothemes

push:
	make
	git add .
	git commit -m "add"
	git push
