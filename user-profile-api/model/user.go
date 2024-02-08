package model 

import (
  "github.com/google/uuid"
  "gorm.io/gorm"
)

type User struct {
  gorm.Model 
  ID uuid.UUID `gorm:"type::uuid;"`
  Username  string  `json:"username"`
  Email   string    `json:"email"`
}

type Users struct {
  Users []User `json:"users"`
}

