DIST_DIR = dist
PUBLIC_DIR = public
SERVER_BIN = server
LAMBDA_ZIP = blog.zip

.PHONY: all
all: build

.PHONY: build
build: clean
	hugo
	mkdir -p dist
	GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -ldflags="-s -w" -o $(DIST_DIR)/$(SERVER_BIN) main.go
	cp -r $(PUBLIC_DIR) $(DIST_DIR)
	cd $(DIST_DIR) && zip -r $(LAMBDA_ZIP) .

.PHONY: dev
dev:
	hugo server --buildDrafts

.PHONY: server
server:
	go run main.go


.PHONY: clean
clean:
	rm -rf $(DIST_DIR) $(PUBLIC_DIR)

.PHONY: re
re: clean build

.PHONY: gzip
gzip:
	gzip -k -9 -r -f $(PUBLIC_DIR)


.PHONY: deploy
deploy:
	sls deploy -s production

.PHONY: tidy
tidy:
	go mod tidy
