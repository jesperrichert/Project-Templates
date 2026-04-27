package config

import (
	"github.com/gin-gonic/gin"
	"go.template/internal/repository"
	"go.template/internal/services"
	"go.template/internal/transport/http"
	"go.template/internal/transport/http/router"
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
