package services

import "log"

type ExampleService struct {
}

func (e *ExampleService) Log(message string) {
	log.Println(message)
}
