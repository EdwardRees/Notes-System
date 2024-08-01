#
# FROM golang:1.21.1-alpine AS build
#
# WORKDIR /app
# COPY . .
#
# RUN go mod download
# # RUN go build -o /user-profile/api .
# RUN go build -v -o api .
#
# FROM alpine:latest
#
# WORKDIR /app
# COPY --from=build api .
# # COPY --from=build /user-profile/api .
#
# CMD ["/app/api"]

FROM golang:1.21.1-alpine

WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -o /api

CMD ["/api"]
