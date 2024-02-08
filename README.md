# Microservice Example

This is a simple Microservice example with four backends. These backends consist of a Notes api, Todo api, User Profile api, and an Auth api. These four backends are written in different languages to demonstrate the ability to develop microservices using different programming languages and allowing them to communicate between each other. Additionally, a client is attached to these services. This client is written in VueJS, using NuxtJS to generate the Server Side Rendering and better user experience. The backend microservices are served using Docker with RabbitMQ serving as a Message Queue. Additionally, gRPCs are used to enable better inter-service communication. The backends will be hosted on a Kubernetes server with the eventual Helm chart included. The frontend will be deployed separately, likely using Vercel or something similar.  

## Tech stack

### Auth Service 

- Rust
- Actix web
- Diesel
- MariaDB

### Notes Service

- TypeScript
- Bun
- Elysia
- DrizzleORM
- PostgreSQL

### Todos Service 

- Python
- FastAPI
- SQLAlchemy
- PostgreSQL

### User Profile Service 

- Go
- Fiber
- Gorm 
- PostgreSQL

### Client 

- VueJS
- NuxtJS 

### Extra Services / Technology

- RabbitMQ
- Docker 
- Kubernetes
- gRPC

