package main

import (
  "fmt"
  "log"
  "strconv"
  "user-profile/api/database"
  "user-profile/api/router"
  "user-profile/api/config"
  "github.com/gofiber/fiber/v2"
  "github.com/gofiber/fiber/v2/middleware/cors"
  "github.com/gofiber/fiber/v2/middleware/logger"
  _ "github.com/lib/pq"
)

func main() {
  database.Connect()

  app := fiber.New()

  app.Use(logger.New())

  app.Use(cors.New())

  router.SetupRoutes(app)

  app.Use(func(c *fiber.Ctx) error {
    return c.SendStatus(404)
  })

  p := config.Config("PORT")
  port, err := strconv.ParseUint(p, 10, 32)

  if err != nil {
    fmt.Println("No port given, defaulting to 8080")
    port = 8080
  }


  log.Printf("Running at port %d\n", port)
  app.Listen(fmt.Sprintf(":%d", port))
}
