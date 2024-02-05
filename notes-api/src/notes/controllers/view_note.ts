import db from '@/db';
import notes from '@/db/schema/notes';

import { eq } from 'drizzle-orm';

const view_note = async (note_id: string) => {

  const note = await db.select().from(notes).where(eq(id, note_id));
  if(!note){
    return {
      status: 404,
      error: "Note not found"
    }
  };
  return {
    status: 200,
    data: note
  }
};

export { view_note };
