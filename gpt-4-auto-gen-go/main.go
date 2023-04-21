package main

import (
	"github.com/gin-gonic/gin"
	_ "github.com/jinzhu/gorm/dialects/postgres"
)

func main() {
	// Initialize the Gin router
	router := gin.Default()

	// Set up routes
	router.POST("/trades", CreateTrade)
	router.GET("/trades", GetTrades)
	router.GET("/trades/:id", GetTradeByID)
	router.PUT("/trades/:id", UpdateTrade)
	router.DELETE("/trades/:id", DeleteTrade)

	// Run the server
	router.Run(":8080")
}
