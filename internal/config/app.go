package config

import (
	"github.com/gin-gonic/gin"
	"go.go-api/internal/repository"
	"go.go-api/internal/services"
	"go.go-api/internal/transport/http"
	"go.go-api/internal/transport/http/router"
	"gorm.io/gorm"
)

type Appconfig struct {
	App *gin.Engine
	DB  *gorm.DB
}

func Build(config *Appconfig) {
	//Register Repositories
	exampleRepository := repository.NewExample()

	//Register Services
	exampleService := services.NewExampleService()

	//Register Controller
	exampleController := http.NewExampleController(config.DB, exampleService, exampleRepository) 

	routeConfig := router.RouterConfig{
		App: config.App, 
		ExampleController: exampleController, 
	} 

	routeConfig.Setup()
}
