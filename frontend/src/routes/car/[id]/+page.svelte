<script lang="ts">
	import { goto } from '$app/navigation';
	export let data;
	const { car, images } = data;

	let mainImage = images.length ? images[0].url : null;

	function selectImage(url: string) {
		mainImage = url;
	}

	function formatPrice(price: string | number) {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(typeof price === 'string' ? parseFloat(price) : price);
	}

	function formatDate(dateString: string) {
		return new Intl.DateTimeFormat('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		}).format(new Date(dateString));
	}
</script>

<div class="max-w-6xl mx-auto px-4 py-8">
	<div class="mb-6">
		<button
			on:click={() => goto('/')}
			class="flex items-center text-gray-400 hover:text-white transition-colors"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-5 w-5 mr-1"
				viewBox="0 0 20 20"
				fill="currentColor"
			>
				<path
					fill-rule="evenodd"
					d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z"
					clip-rule="evenodd"
				/>
			</svg>
			Back to Listings
		</button>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
		<div class="lg:col-span-2 space-y-6">
			<div class="bg-gray-700 rounded-xl overflow-hidden h-80 flex items-center justify-center">
				{#if mainImage}
					<img src={mainImage} alt="{car.make} {car.model}" class="w-full h-full object-cover" />
				{:else}
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-16 w-16 text-gray-500"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="1.5"
							d="M19 16v3a2 2 0 01-2 2H7a2 2 0 01-2-2v-3m14-6v-3a2 2 0 00-2-2H7a2 2 0 00-2 2v3m14 0H5"
						/>
					</svg>
				{/if}
			</div>

			{#if images.length > 0}
				<div class="grid grid-cols-4 gap-3">
					{#each images as img}
						<div
							class="bg-gray-700 rounded-lg overflow-hidden aspect-square cursor-pointer hover:opacity-80 transition-opacity"
							on:click={() => selectImage(img.url)}
						>
							<img src={img.url} alt="thumbnail" class="w-full h-full object-cover" />
						</div>
					{/each}
				</div>
			{/if}

      {#if car.description}
        <div class="bg-gray-800 rounded-xl p-6">
          {car.description}
        </div>
      {/if}
		</div>

		<div class="space-y-6">
			<div class="bg-gray-800 rounded-xl p-6">
				<h1 class="text-2xl font-bold">{car.make} {car.model}</h1>
				<p class="text-gray-400 mt-1">{car.year} Model</p>
				<div class="mt-6">
					<div class="text-3xl font-bold text-white">{formatPrice(car.price)}</div>
				</div>
				<div class="mt-6 pt-6 border-t border-gray-700">
					<h3 class="text-lg font-medium mb-3">Specifications</h3>
					<div class="space-y-3">
						<div class="flex justify-between">
							<span class="text-gray-400">Make:</span>
							<span class="font-medium">{car.make}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400">Model:</span>
							<span class="font-medium">{car.model}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400">Year:</span>
							<span class="font-medium">{car.year}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400">Mileage:</span>
							<span class="font-medium">{car.mileage}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400">Engine size:</span>
							<span class="font-medium">{car.engine}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400">Transmission:</span>
							<span class="font-medium">{car.transmission}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400">Listed:</span>
							<span class="font-medium">{formatDate(car.created_at)}</span>
						</div>
					</div>
				</div>
			</div>

      <div>
        <a
          href="https://t.me/lamp_y"
          class="block bg-purple-600 text-white p-6 rounded-lg"
          target="_blank">
          Contact manager
        </a>
      </div>
		</div>
	</div>
</div>
