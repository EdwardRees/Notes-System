import db from '@/db';
import notes from '@/db/schema/notes';
import { eq } from 'drizzle-orm';

const view_notes = async (user_id: string) => {
  const notes = await db.query.notes.findMany({
    where: eq(user_id, user_id)
  });

  if(!notes){
    return { status: 404, error: `Notes not found`}
  }
  return { 
    status: 200,
    data: notes
  }
}

export { view_notes };
