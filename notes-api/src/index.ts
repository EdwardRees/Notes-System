import { Elysia } from "elysia";
import { notes_routes } from '@/routes';
import { jwt } from '@elysiajs/jwt';
import db from '@/db';

const port = parseInt(process.env.PORT || "3000");

const app = new Elysia()
  .use(
    jwt({
      name: 'jwt',
      secret: process.env.JWT_SECRET
    })
  )
  .get("/v2/", () => "Hello from the Notes API")
  .get("/v2/health", () => "Health!")
  .group("/v2/notes", (app) => notes_routes(app))
  .listen(port);

console.info(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);

process.on("SIGINT", () => {
  console.info("Shutting down...");
  app.stop();
  process.exit(0);
});
