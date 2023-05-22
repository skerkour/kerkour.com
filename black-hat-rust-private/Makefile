BOOK_FILE = black_hat_rust
BOOK_TITLE = Black Hat Rust
CHAPTERS_FILES = src/settings.txt src/copyright.md src/early_access_bonuses.md src/contact.md src/preface.md \
	src/ch_01.md \
	src/ch_02.md \
	src/ch_03.md \
	src/ch_04.md \
	src/ch_05.md \
	src/ch_06.md \
	src/ch_07.md \
	src/ch_08.md \
	src/ch_09.md \
	src/ch_10.md \
	src/ch_11.md \
	src/ch_12.md \
	src/ch_13.md \
	src/ch_14.md

PIRATE_FILES = src/settings.txt src/copyright.md src/early_access_bonuses.md src/contact.md src/preface.md \
	src/ch_01.md src/pirate.md

COVER = src/assets/black_hat_rust_cover.png
DATE := $(shell date "+%B %e, %Y")
VERSION = v2022.56
DIST_DIR = ebooks
DOCKER_IMAGE = localhost/skerkour/ebook
PIRATE_DIR = ebooks/pirate


.PHONY: all
all: pdf epub epubcheck mobi zip azw3

.PHONY: pdf
pdf:
	pandoc $(CHAPTERS_FILES) \
	--strip-comments \
	--resource-path=src \
	--output=$(DIST_DIR)/$(BOOK_FILE)_content.pdf \
	--pdf-engine=xelatex \
	--table-of-contents --toc-depth=2 \
	--number-sections \
	--top-level-division=chapter \
	--include-in-header src/inline_code.tex \
	-V fontsize=12pt \
	-V documentclass=report \
	-V linkcolor:blue \
	--highlight-style ./src/tango_mod.theme \
	-M date="$(VERSION)"
	pdftk src/assets/black_hat_rust_cover.pdf $(DIST_DIR)/$(BOOK_FILE)_content.pdf cat output $(DIST_DIR)/$(BOOK_FILE).pdf
	rm $(DIST_DIR)/$(BOOK_FILE)_content.pdf


.PHONY: epub
epub:
	pandoc $(CHAPTERS_FILES) \
	--strip-comments \
	--resource-path=src \
	--output=$(DIST_DIR)/$(BOOK_FILE).epub \
	--table-of-contents --toc-depth=2 \
	--top-level-division=chapter \
	--number-sections \
	--listings \
	--standalone \
	--epub-cover-image=$(COVER) \
	--metadata title="$(BOOK_TITLE)" \
	--highlight-style ./src/tango_mod.theme \
	--css ./src/epub.css \
	-M date="$(DATE)"


.PHONY: epubcheck
epubcheck:
	java -jar /usr/bin/epubcheck ebooks/black_hat_rust.epub


.PHONY: mobi
mobi:
	ebook-convert $(DIST_DIR)/$(BOOK_FILE).epub $(DIST_DIR)/$(BOOK_FILE).mobi --cover $(COVER)

.PHONY: azw3
azw3:
	ebook-convert $(DIST_DIR)/$(BOOK_FILE).epub $(DIST_DIR)/$(BOOK_FILE).azw3 --cover $(COVER)


.PHONY: docker
docker:
	docker build -t $(DOCKER_IMAGE):latest .


.PHONY: zip
zip:
	zip $(DIST_DIR)/$(BOOK_FILE).zip $(DIST_DIR)/$(BOOK_FILE).mobi $(DIST_DIR)/$(BOOK_FILE).epub $(DIST_DIR)/$(BOOK_FILE).pdf







####################################################################################################
## Pirate
####################################################################################################

.PHONY: pirate
pirate: pdf_pirate epub_pirate epubcheck_pirate mobi_pirate azw3_pirate


.PHONY: pdf_pirate
pdf_pirate:
	pandoc $(PIRATE_FILES) \
	--strip-comments \
	--resource-path=src \
	--output=$(PIRATE_DIR)/$(BOOK_FILE)_content.pdf \
	--pdf-engine=xelatex \
	--table-of-contents --toc-depth=2 \
	--number-sections \
	--top-level-division=chapter \
	--include-in-header src/inline_code.tex \
	-V fontsize=12pt \
	-V documentclass=report \
	-V linkcolor:blue \
	--highlight-style ./src/tango_mod.theme \
	-M date="$(VERSION)"
	pdftk src/assets/black_hat_rust_cover.pdf $(PIRATE_DIR)/$(BOOK_FILE)_content.pdf cat output $(PIRATE_DIR)/$(BOOK_FILE).pdf
	rm $(PIRATE_DIR)/$(BOOK_FILE)_content.pdf


.PHONY: epub_pirate
epub_pirate:
	pandoc $(PIRATE_FILES) \
	--strip-comments \
	--resource-path=src \
	--output=$(PIRATE_DIR)/$(BOOK_FILE).epub \
	--table-of-contents --toc-depth=2 \
	--top-level-division=chapter \
	--number-sections \
	--listings \
	--standalone \
	--epub-cover-image=$(COVER) \
	--metadata title="$(BOOK_TITLE)" \
	--highlight-style ./src/tango_mod.theme \
	--css ./src/epub.css \
	-M date="$(DATE)"


.PHONY: epubcheck_pirate
epubcheck_pirate:
	java -jar /usr/bin/epubcheck $(PIRATE_DIR)/black_hat_rust.epub


.PHONY: mobi_pirate
mobi_pirate:
	ebook-convert $(PIRATE_DIR)/$(BOOK_FILE).epub $(PIRATE_DIR)/$(BOOK_FILE).mobi --cover $(COVER)

.PHONY: azw3_pirate
azw3_pirate:
	ebook-convert $(PIRATE_DIR)/$(BOOK_FILE).epub $(PIRATE_DIR)/$(BOOK_FILE).azw3 --cover $(COVER)
