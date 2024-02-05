import { get_user } from '@/util';
import { view_note } from '@/notes/controllers/view_note';

const view_note_handler = async (
  headers: { "x-access-token" : string }, 
  note_id: string
) => {
  const { user_id } = await get_user(headers);
  if(!user_id){
    return {
      status: 401,
      error: "Unauthorized"
    }
  };
  return await view_note(note_id); 
}

export { view_note_handler };
