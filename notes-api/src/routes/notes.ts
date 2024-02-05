import { Elysia, t } from 'elysia';
import { new_note_handler, view_note_handler } from '@/notes/routes'; 

const notes_routes = (app: Elysia<any>): Elysia => app
  .post("/", ({ headers, body }) => new_note_handler(headers, body), {
    body: t.Object({
      title: t.String(),
      description: t.String(),
      categories: t.String(),
      tags: t.String()
    }),
    headers: t.Object({
      "x-access-token": t.String()
    })
  })
  .get("/note_id", ({ headers, params }) => view_note_handler(headers, params.note_id), {
    headers: t.Object({
      "x-access-token": t.String()
    }),
    params: t.Object({
      note_id: t.String()
    })
  })
  .get("/user/:user_id", ({ headers, params }) => view_notes_handler(headers, params.user_id),  {
    headers: t.Object({
      "x-access-token": t.String()
    }),
    params: t.Object({
      user_id: t.String()
    })
  })


export { notes_routes };
