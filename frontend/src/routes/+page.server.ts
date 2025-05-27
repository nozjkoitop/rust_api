import { API_URL } from "$lib";
import type { PageServerLoad } from "./$types";

interface Car {
  id: number;
  make: string;
  model: string;
  year: number;
  price: string;
  created_at: string;
}

interface Picture {
  id: number;
  car_id: number;
  url: string;
  created_at: string;
}

interface CarWithImage extends Car {
  image: string | null;  
}

export const load: PageServerLoad = async ({ fetch }) => {
  const carsRes = await fetch(`${API_URL}/cars`, {
    headers: { "Content-Type": "application/json" },
  });
  if (!carsRes.ok) {
    console.error("Failed to fetch cars:", carsRes.status, await carsRes.text());
    return { cars: [] };
  }
  const cars: Car[] = await carsRes.json();

  const carsWithImage: CarWithImage[] = await Promise.all(
    cars.map(async (car) => {
      try {
        const listRes = await fetch(
          `${API_URL}/cars/${car.id}/images`,
          { headers: { "Content-Type": "application/json" } }
        );
        if (!listRes.ok) throw new Error("no images");
        const pics: Picture[] = await listRes.json();
        const first = pics[0];
        if (!first) throw new Error("empty");

        console.log("First image:", first);


        return { ...car,  image: first
      ? `/image/${first.id}`
      : null };
      } catch {
        return { ...car, image: null };
      }
    })
  );

  console.log("Cars with images:", carsWithImage);

  return { cars: carsWithImage };
};
