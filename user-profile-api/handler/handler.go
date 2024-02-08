package handler 

import (
  "user-profile/api/database"
  "user-profile/api/model"
  "github.com/gofiber/fiber/v2"
  "github.com/google/uuid"
)

func Welcome(c *fiber.Ctx) error {
  return c.Status(200).JSON(fiber.Map {"status": "success", "message": "Welcome to the User Profile Service!", "data": "Welcome!"})
}

func Health(c *fiber.Ctx) error {
  return c.Status(200).JSON(fiber.Map { "status": "success", "message": "Healthy!", "data": "Healthy!"})
}

func CreateUser(c *fiber.Ctx) error {

  db := database.DB.Db
  user := new(model.User)

  err := c.BodyParser(user)

  if err != nil {
    return c.Status(500).JSON(fiber.Map { "status": "error", "message": "InternalServerError: Invalid Request Body", "data": err})
  }

  err = db.Create(&user).Error 

  if err != nil {
    return c.Status(500).JSON(fiber.Map{"status": "error", "message": "InternalServerError: Could not create user", "data": err})
  }

  return c.Status(201).JSON(fiber.Map{"status": "success", "message": "User successfully created", "data": user})

}

func GetAllUsers(c *fiber.Ctx) error {
  db := database.DB.Db 
  var users []model.User 

  db.Find(&users)

  if len(users) == 0 {
    return c.Status(404).JSON(fiber.Map{"status": "error", "message": "Users not found", "data": nil})
  }

  return c.Status(200).JSON(fiber.Map{"status": "success", "message": "Users found", "data": users})
}

// GetSingleUser from db
func GetSingleUser(c *fiber.Ctx) error {
  db := database.DB.Db
  // get id params
  id := c.Params("id")
  var user model.User
  // find single user in the database by id
  db.Find(&user, "id = ?", id)
  if user.ID == uuid.Nil {
    return c.Status(404).JSON(fiber.Map{"status": "error", "message": "User not found", "data": nil})
  }
  return c.Status(200).JSON(fiber.Map{"status": "success", "message": "User Found", "data": user})
}


// update a user in db
func UpdateUser(c *fiber.Ctx) error {
  type updateUser struct {
    Username string `json:"username"`
  }
  db := database.DB.Db
  var user model.User
  // get id params
  id := c.Params("id")
  // find single user in the database by id
  db.Find(&user, "id = ?", id)
  if user.ID == uuid.Nil {
    return c.Status(404).JSON(fiber.Map{"status": "error", "message": "User not found", "data": nil})
  }
  var updateUserData updateUser
  err := c.BodyParser(&updateUserData)
  if err != nil {
    return c.Status(500).JSON(fiber.Map{"status": "error", "message": "Something's wrong with your input", "data": err})
  }
  user.Username = updateUserData.Username
  // Save the Changes
  db.Save(&user)
  // Return the updated user
  return c.Status(200).JSON(fiber.Map{"status": "success", "message": "users Found", "data": user})
}

// delete user in db by ID
func DeleteUserByID(c *fiber.Ctx) error {
  db := database.DB.Db
  var user model.User
  // get id params
  id := c.Params("id")
  // find single user in the database by id
  db.Find(&user, "id = ?", id)
  if user.ID == uuid.Nil {
    return c.Status(404).JSON(fiber.Map{"status": "error", "message": "User not found", "data": nil})
  }
  err := db.Delete(&user, "id = ?", id).Error
  if err != nil {
    return c.Status(404).JSON(fiber.Map{"status": "error", "message": "Failed to delete user", "data": nil})
  }
  return c.Status(200).JSON(fiber.Map{"status": "success", "message": "User deleted"})
}
