import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { API_URL } from "$lib";

export const load: PageServerLoad = async ({ fetch, params }) => {
    const resCar = await fetch(`${API_URL}/cars/${params.id}`, {
        headers: { "Content-Type": "application/json" },
    });
    if (!resCar.ok) throw redirect(303, "/");
    const car = await resCar.json();

    const resPics = await fetch(`${API_URL}/cars/${params.id}/images`, {
        headers: {
            "Content-Type": "application/json",
        },
    });

    const pics: { id: number; url: string }[] = resPics.ok
        ? await resPics.json()
        : [];

    const images = pics.map((pic) => ({
        id: pic.id,
        url: `/image/${pic.id}`
    }));

    return { car, images };
};
