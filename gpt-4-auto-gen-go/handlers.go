package main

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
)

// CreateTrade handler
func CreateTrade(c *gin.Context) {
	var trade Trade
	if err := c.ShouldBindJSON(&trade); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	if err := db.Create(&trade).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, trade)
}

// GetTrades handler
func GetTrades(c *gin.Context) {
	var trades []Trade
	if err := db.Find(&trades).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, trades)
}

// GetTradeByID handler
func GetTradeByID(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid trade ID"})
		return
	}

	var trade Trade
	if err := db.First(&trade, id).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Trade not found"})
		return
	}

	c.JSON(http.StatusOK, trade)
}

// UpdateTrade handler
func UpdateTrade(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid trade ID"})
		return
	}

	var trade Trade
	if err := db.First(&trade, id).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Trade not found"})
		return
	}

	var updatedTrade Trade
	if err := c.ShouldBindJSON(&updatedTrade); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	trade.Value = updatedTrade.Value
	trade.Date = updatedTrade.Date
	trade.Trader = updatedTrade.Trader

	if err := db.Save(&trade).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, trade)
}

// DeleteTrade handler
func DeleteTrade(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid trade ID"})
		return
	}

	var trade Trade
	if err := db.First(&trade, id).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Trade not found"})
		return
	}

	if err := db.Delete(&trade).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusNoContent, nil)
}
