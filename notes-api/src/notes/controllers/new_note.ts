import db from '@/db';
import notes from '@/db/schema/notes';

import { randomUUID } from 'crypto';

const new_note = async (body: any) => {
  const {
    user_id,
    title,
    description,
    categories,
    tags
  } = body;

  const note_id = randomUUID();

  const note = await db.insert(notes).values({
    id: note_id,
    user_id: user_id,
    title: title,
    description: description,
    categories: categories,
    tags: tags
  }).returning();

  if(!note){
    return { status: 500, error: `DBError: Failed to insert new note`}
  }

  return { status: 201, data: note };
}

export { new_note };
