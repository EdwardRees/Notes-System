import { get_user } from "@/util";
import { new_note } from '@/notes/controllers/new_note';

const new_note_handler = async (headers: { "x-access-token": string }, body: any) => {
  const { user_id } = await get_user(headers);
  if(!user_id){
    return {
      status: 401,
      error: "Unauthorized"
    }
  };
  let request_body = {
    user_id: user_id,
    ...body
  };
  return await new_note(request_body);
}

export { new_note_handler };
