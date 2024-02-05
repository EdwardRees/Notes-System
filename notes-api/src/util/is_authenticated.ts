import { Elysia } from 'elysia';

export const isAuthenticated = (app: Elysia) => 
  app.derive(async ({ headers, jwt, set }) => {
    if(!Object(headers).keys().includes('x-access-token')){
      set.status = 401;
      return {
        status: 401,
        error: 'Unauthorized'
      }
    };
    const { id } = await jwt.verify(headers['x-access-token']);
    if(!id){
      set.status = 401;
      return {
        status; 401,
        error: "Unauthorized"
      }
    } else {
      return {
        id
      }
    }
  });
