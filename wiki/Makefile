.PHONY: dev build release

build:
	mkdocs build

dev:
	mkdocs serve -a 127.0.0.1:8001

release:
	git checkout master
	git merge dev
	git push
	git checkout dev
