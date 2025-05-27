import type { HandleFetch } from "@sveltejs/kit";

export const handleFetch: HandleFetch = async ({ request, event, fetch }) => {
  const token = event.cookies.get("jwt");

  if (token) {
    request.headers.set("Authorization", `Bearer ${token}`);
  }


  let response = await fetch(request);

  return response;
};
