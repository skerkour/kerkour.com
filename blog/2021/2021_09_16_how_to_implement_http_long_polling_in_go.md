+++
date = 2021-09-16T06:00:00Z
title = "How to implement HTTP Long Polling in Go"
type = "post"
tags = ["go", "programming", "tutorial", "webdev"]
authors = ["Sylvain Kerkour"]
url = "/go-http-long-polling"

[extra]
lang = "en"

comment ="""
"""
+++



We will implement a simple chat server, as chat is the textbook application that benefits the most from long polling.

**There are 3 tricks to make this implementation efficient, so stay attentive ;)**


### The Chat Service

The Chat Service is an object that encapsulates all our business logic. To keep the example simple, we use a simple `map`, but I commented the appropriate SQL queries.

**chat.go**
```go
package main

import (
	"sync"
	"time"

	"github.com/google/uuid"
)

type Message struct {
	ID       uuid.UUID `json:"id"`
	CreateAt time.Time `json:"created_at"`
	Body     string    `json:"body"`
}

type ChatService struct {
	mutex    sync.RWMutex
	messages map[uuid.UUID]Message
}

func (service *ChatService) CreateMessage(body string) (Message, error) {
	message := Message{
		ID:       uuid.New(),
		CreateAt: time.Now().UTC(),
		Body:     body,
	}

	// TODO: insert in DB
	// "INSERT INTO messages
	//         (id, created_at, body)
	//         VALUES ($1, $2, $3)"

	service.mutex.Lock()
	service.messages[message.ID] = message
	service.mutex.Unlock()

	return message, nil
}

func (service *ChatService) FindMessages(after *uuid.UUID) ([]Message, error) {
	if after == nil {
		after = &uuid.UUID{} // create a zero UUID
	}
	// TODO: fetch in DB
	// "SELECT * FROM messages WHERE id > $1"

	service.mutex.RLock()
	messages := make([]Message, len(service.messages))
	for _, message := range service.messages {
		messages = append(messages, message)
	}
	service.mutex.RUnlock()

	return messages, nil
}
```


### Long Polling

Long polling is in reality a simple loop with `time.Sleep(time.Second)`. Thanks to Go's runtime, it's extremely efficient: by using `time.Sleep` an active connection will barely use any resources when waiting.

If new data is found, we immediately return with the new data. Else, we wait one more second.

After 10 seconds, we return empty data.

```go
func (server *Server) FindMessagesHandler(w http.ResponseWriter, req *http.Request) {
	after := req.URL.Query().Get("after")
	if after == "" {
		after = "00000000-0000-0000-0000-000000000000"
	}

	afterUUID, err := uuid.Parse(after)
	if err != nil {
		httpError(w, err)
		return
	}

	// long polling: 10 secs
	for i := 0; i < 10; i++ {
		messages, err := server.chatService.FindMessages(&afterUUID)
		if err != nil {
			httpError(w, err)
			return
		}

		if len(messages) != 0 {
			httpOk(w, messages)
			return
		}

		time.Sleep(time.Second)
	}

	messages := []Message{}
	httpOk(w, messages)
}
```


### Web Server

Finally, the boilerplate to run the web server:

**main.go**
```go
package main

import (
	"encoding/json"
	"io"
	"io/ioutil"
	"net/http"
	"sync"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/google/uuid"
)

type Server struct {
	chatService *ChatService
}

type CreateMessage struct {
	Body string `json:"body"`
}

func main() {
	r := chi.NewRouter()
	chatService := ChatService{
		mutex:    sync.RWMutex{},
		messages: map[uuid.UUID]Message{},
	}

	server := Server{
		chatService: &chatService,
	}

	r.Get("/messages", server.FindMessagesHandler)
	r.Post("/messages", server.CreateMessageHandler)

	http.ListenAndServe(":8080", r)
}

func (server *Server) CreateMessageHandler(w http.ResponseWriter, req *http.Request) {
	var input CreateMessage

	body, err := ioutil.ReadAll(io.LimitReader(req.Body, 10000))
	if err != nil {
		httpError(w, err)
		return

	}
	if err := req.Body.Close(); err != nil {
		httpError(w, err)
		return
	}
	if err := json.Unmarshal(body, &input); err != nil {
		httpError(w, err)
		return
	}

	message, err := server.chatService.CreateMessage(input.Body)
	if err != nil {
		httpError(w, err)
		return
	}

	httpOk(w, message)
}

func httpError(w http.ResponseWriter, err error) {
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusInternalServerError)
	json.NewEncoder(w).Encode(err)
}

func httpOk(w http.ResponseWriter, resp interface{}) {
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(resp)
}
```


You can run this long polling server as follows:


In shell 1:
```shell
$ go run *.go
```

In shell 2:
```shell
$ curl http://localhost:8080/messages
```

In shell 3:
```shell
$ curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"body":"Hello World"}' \
  http://localhost:8080/messages
```



## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/go_http_long_polling) (please don't forget to star the repo 🙏).

