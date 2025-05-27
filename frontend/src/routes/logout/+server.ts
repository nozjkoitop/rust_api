import { redirect } from "@sveltejs/kit";

export const GET = async ({ cookies }) => {
  cookies.delete("jwt", {
    path: "/",          
    httpOnly: true,
    sameSite: "strict",
    secure: true      
  });

  throw redirect(303, "/login");
};
