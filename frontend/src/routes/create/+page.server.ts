import { API_URL } from '$lib';
import { fail } from '@sveltejs/kit';


export const actions = {
  createCar: async ({ request, fetch }) => {
    const data = await request.formData();
    const make = data.get('make');
    const model = data.get('model');
    const yearStr = data.get('year');
    const priceStr = data.get('price');

    if (!make || !model || !yearStr || !priceStr) {
      return fail(400, { message: 'All fields are required' });
    }

    const year = parseInt(yearStr.toString(), 10);
    if (isNaN(year) || year < 1900 || year > new Date().getFullYear() + 1) {
      return fail(400, { message: 'Please enter a valid year' });
    }

    const price = parseFloat(priceStr.toString());
    if (isNaN(price) || price < 0) {
      return fail(400, { message: 'Please enter a valid price' });
    }

    let car: { id: number };
    try {
      const resp = await fetch(`${API_URL}/cars`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ make, model, year, price })
      });

      if (!resp.ok) {
        const txt = await resp.text();
        return fail(resp.status, { message: txt });
      }

      car = await resp.json();
    } catch (err) {
      console.error('Error creating car:', err);
      return fail(500, { message: 'An unexpected error occurred. Please try again later.' });
    }

    const rawFiles = data.getAll('file-upload');
    const files = rawFiles.filter(
      (f): f is File => f instanceof File && f.size > 0
    );

    if (files.length > 0) {
      try {
        await Promise.all(
          files.map(async (file) => {
            const form = new FormData();
            form.append('file', file, file.name);

            const imgResp = await fetch(
              `${API_URL}/cars/${car.id}/image`,
              {
                method: 'POST',
                body: form,
              }
            );

            if (!imgResp.ok) {
              throw new Error(await imgResp.text());
            }
          })
        );
      } catch (uploadErr) {
        console.error('Error uploading images:', uploadErr);
        return fail(500, {
          message:
            'Car was created, but uploading images failed. Please try again later.',
        });
      }
    }

    return { success: true, carId: car.id };
  }
};
