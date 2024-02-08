package router 

import (
  "user-profile/api/handler"
  "github.com/gofiber/fiber/v2"
)

func SetupRoutes(app *fiber.App) {
  api := app.Group("/v1/")
  v1 := api.Group("user")

  api.Get("/", handler.Welcome)

  api.Get("/health", handler.Health)

  v1.Get("/", handler.GetAllUsers)
  v1.Get("/:id", handler.GetSingleUser)
  v1.Post("/", handler.CreateUser)
  v1.Put("/:id", handler.UpdateUser)
  v1.Delete("/:id", handler.DeleteUserByID)
}
