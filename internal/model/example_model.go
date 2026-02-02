package model

type Example struct {
	ID    int `gorm:"primaryKey"`
	Name  string
	Email string
}

type ExampleRequest struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

type ExampleResponse struct {
	ID    int    `json:"id"`
	Name  string `json:"name"`
	Email string `json:"email"`
}
