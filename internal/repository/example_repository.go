package repository

import "go.template/internal/model"

type ExampleRepository struct {
	Repository[model.Example]
}

func NewExample() *ExampleRepository {
	return &ExampleRepository{}
}
