####################################################################################################
## Build server
####################################################################################################
# see https://medium.com/@chemidy/create-the-smallest-and-secured-golang-docker-image-based-on-scratch-4752223b7324
FROM golang:alpine AS builder_go

# Install git + SSL ca certificates.
# Git is required for fetching the dependencies.
# Ca-certificates is required to call HTTPS endpoints.
RUN apk update && apk add --no-cache git ca-certificates make gcc libc-dev
RUN update-ca-certificates

# Create appuser
ENV USER=skerkour
ENV UID=10001

# See https://stackoverflow.com/a/55757473/12429735RUN
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /hugo
RUN git clone https://github.com/gohugoio/hugo.git
WORKDIR /hugo/hugo
RUN go install --tags extended


WORKDIR /skerkour

COPY . .

# Using go mod with go >= 1.11
RUN go mod download
RUN go mod verify

RUN make build

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder_go /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder_go /etc/passwd /etc/passwd
COPY --from=builder_go /etc/group /etc/group

WORKDIR /skerkour

# Copy our builds
COPY --from=builder_go /skerkour/dist/ ./

# Use an unprivileged user.
USER skerkour:skerkour

EXPOSE 8080 8443
CMD ["/skerkour/server"]

LABEL maintainer="skerkour <https://kerkour.fr>"
LABEL homepage=https://kerkour.fr
LABEL org.opencontainers.image.name=kerkour.fr
LABEL repository=https://github.com/skerkour/kerkour.fr
