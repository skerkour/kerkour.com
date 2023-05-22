all: pdf epub kindle html examples

BOOK_TITLE = My\ awesome\ book

dist:
	@mkdir -p dist

.PHONY: pdf
pdf: dist/${BOOK_TITLE}.pdf
	@echo '✅  PDF'

.PHONY: epub
epub: dist/${BOOK_TITLE}.epub
	@echo '✅  ePub'

kindle: dist/${BOOK_TITLE}.mobi
	@echo '✅  Kindle'

html: dist/${BOOK_TITLE}.html
	@echo '✅  HTML'

examples: dist/Examples.zip
	@echo '✅  Examples'

.PHONY: dist/${BOOK_TITLE}.pdf
dist/${BOOK_TITLE}.pdf: dist *.md
	pandoc *.md \
	--output=dist/${BOOK_TITLE}.pdf \
	--table-of-contents \
	--include-in-header inline_code.tex \
	--include-before-body cover.tex \
	--number-sections \
	-V fontsize=12pt \
	-V documentclass=report 
	# -H header.tex
	# --highlight-style dracula.theme \
	# 	--highlight-style=monochrome \
	# 	--pdf-engine=xelatex \
	# 	--template eisvogel \
	# 	--pdf-engine=xelatex \
	# 	# --listings \



dist/${BOOK_TITLE}.html: dist content/*.md images/* resources/dracula.theme resources/header.html
	@pandoc content/foreword.md content/ch*.md content/resources.md \
	--output=dist/${BOOK_TITLE}.html \
	--highlight-style resources/dracula.theme \
	--standalone \
	-H resources/header.html \
	--metadata title="A React Developer’s Guide to Hooks"

.PHONY: dist/${BOOK_TITLE}.epub
dist/${BOOK_TITLE}.epub: dist *.md
	pandoc ch*.md \
	--output=dist/${BOOK_TITLE}.epub \
	--highlight-style dracula.theme \
	--standalone \
	--epub-cover-image=cover.png \
	--toc --toc-depth=1 \
	--metadata title="A React Developer’s Guide to Hooks"

	# -c resources/epub.css \
	# --epub-metadata=resources/metadata.xml \


dist/${BOOK_TITLE}.mobi: dist/${BOOK_TITLE}.epub
	@kindlegen dist/${BOOK_TITLE}.epub -o ${BOOK_TITLE}.mobi >/dev/null 2>&1

dist/Examples.zip: examples/**/*
	@git archive --prefix=examples/ -o dist/Examples.zip HEAD:examples/

clean:
	@rm -rf dist