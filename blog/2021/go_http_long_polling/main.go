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
