package main

import (
	"time"

	"github.com/jinzhu/gorm"
)

type Trade struct {
	ID     uint      `gorm:"primary_key" json:"id"`
	Value  float64   `json:"value"`
	Date   time.Time `json:"date"`
	Trader string    `json:"trader"`
}

var db *gorm.DB

func init() {
	var err error
	db, err = gorm.Open("postgres", "host=localhost user=postgres password=postgres dbname=postgres_database sslmode=disable")
	if err != nil {
		panic("failed to connect to the database")
	}

	db.AutoMigrate(&Trade{})
}
