import { get_user } from '@/util';
import { view_notes } from '@/notes/controllers/view_notes';

const view_notes_handler = async (
  headers: { 'x-access-token': string },
  user_id: string
) => {
  const { user_id: personal_id } = await get_user(headers);
  if(!personal_id){
    return {
      status: 401,
      error: "Unauthorized"
    }
  }
  return await view_notes(user_id);
}

export { view_notes_handler };
