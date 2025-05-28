import type { ServerLoad } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";

export const load: ServerLoad = async ({ url, cookies }) => {
  const jwt = cookies.get("jwt");

  // if (!jwt && !["/login", "/register"].includes(url.pathname)) {
  //   throw redirect(303, "/login");
  // }

  return {
    loggedIn: Boolean(jwt),
  };
};
