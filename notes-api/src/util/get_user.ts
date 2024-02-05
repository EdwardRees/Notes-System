import { jwt } from "@elysiajs/jwt";

const get_user = async (headers: {'x-access-token': string}) => {
  const { id } = await jwt.verify(headers['x-access-token']);
  if(!id){
    return {
      user_id: null
    }
  }

  return { user_id: id };
}

export { get_user };
