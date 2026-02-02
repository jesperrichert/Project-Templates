package config

import (
	"github.com/gin-gonic/gin"
	"go.template/internal/repository"
	"go.template/internal/services"
	"go.template/internal/transport/http"
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
	exampleService := services.N

	//Register Controller
	exampleController := http.ExampleController

}
