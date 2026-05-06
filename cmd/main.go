package main

import "go.go-api/internal/config"


func main() { 
	app := config.NewGin() 
	db := config.NewDatabase() 

	config.Build(&config.Appconfig{
		App: app,  
		DB: db,
	}) 

	app.Run(":3000")
}