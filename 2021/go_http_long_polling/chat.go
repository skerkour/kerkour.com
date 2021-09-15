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
