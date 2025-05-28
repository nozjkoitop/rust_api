<script lang="ts">
	export let data;

	let cars = data.cars || [];
	let filteredCars = [...cars];

	$: makes = [...new Set(cars.map((car) => car.make))];
	$: models = [...new Set(cars.map((car) => car.model))];
	$: years = [...new Set(cars.map((car) => car.year))].sort((a, b) => a - b);

	let selectedMake = '';
	let selectedModel = '';
	let selectedYear = '';
	let sortBy = 'default';

	$: {
		filteredCars = cars.filter((car) => {
			return (
				(!selectedMake || car.make === selectedMake) &&
				(!selectedModel || car.model === selectedModel) &&
				(!selectedYear || car.year === parseInt(selectedYear))
			);
		});

		if (sortBy === 'priceAsc') {
			filteredCars.sort((a, b) => parseFloat(a.price) - parseFloat(b.price));
		} else if (sortBy === 'priceDesc') {
			filteredCars.sort((a, b) => parseFloat(b.price) - parseFloat(a.price));
		} else if (sortBy === 'yearAsc') {
			filteredCars.sort((a, b) => a.year - b.year);
		} else if (sortBy === 'yearDesc') {
			filteredCars.sort((a, b) => b.year - a.year);
		}
	}

	function resetFilters() {
		selectedMake = '';
		selectedModel = '';
		selectedYear = '';
		sortBy = 'default';
	}

	function formatPrice(price: string | number | bigint) {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(typeof price === 'string' ? parseFloat(price) : price);
	}
</script>

<div class="max-w-6xl mx-auto px-4 py-8">
	<div class="mb-8">
		<img
			src="/car.png"
			alt="Car Banner"
			class="w-full h-120 object-cover object-bottom rounded-xl shadow-md"
		/>
	</div>

	<div class="flex justify-between items-center mb-8">
		<div>
			<h1 class="text-3xl font-bold">Available Cars</h1>
			<p class="text-gray-400 mt-1">Browse our inventory</p>
		</div>
	</div>

	<div class="bg-gray-800 p-6 rounded-xl mb-8 shadow-lg">
		<h2 class="text-lg font-semibold mb-4">Filter & Sort</h2>
		<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
			<div>
				<label for="make-filter" class="block text-sm font-medium text-gray-300 mb-1">Make</label>
				<select
					id="make-filter"
					bind:value={selectedMake}
					class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				>
					<option value="">All Makes</option>
					{#each makes as make}
						<option value={make}>{make}</option>
					{/each}
				</select>
			</div>

			<div>
				<label for="model-filter" class="block text-sm font-medium text-gray-300 mb-1">Model</label>
				<select
					id="model-filter"
					bind:value={selectedModel}
					class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				>
					<option value="">All Models</option>
					{#each models as model}
						<option value={model}>{model}</option>
					{/each}
				</select>
			</div>

			<div>
				<label for="year-filter" class="block text-sm font-medium text-gray-300 mb-1">Year</label>
				<select
					id="year-filter"
					bind:value={selectedYear}
					class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				>
					<option value="">All Years</option>
					{#each years as year}
						<option value={year}>{year}</option>
					{/each}
				</select>
			</div>

			<div>
				<label for="sort-by" class="block text-sm font-medium text-gray-300 mb-1">Sort By</label>
				<select
					id="sort-by"
					bind:value={sortBy}
					class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				>
					<option value="default">Default</option>
					<option value="priceAsc">Price: Low to High</option>
					<option value="priceDesc">Price: High to Low</option>
					<option value="yearAsc">Year: Oldest First</option>
					<option value="yearDesc">Year: Newest First</option>
				</select>
			</div>
		</div>

		<button
			on:click={resetFilters}
			class="mt-6 px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors"
		>
			Reset Filters
		</button>
	</div>

	<p class="mb-6 text-gray-400">
		Showing {filteredCars.length} of {cars.length} cars
	</p>

	{#if filteredCars.length > 0}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			{#each filteredCars as car (car.id)}
				<div
					class="bg-gray-800 rounded-xl shadow-lg overflow-hidden border border-gray-700 hover:border-gray-600 transition-all hover:translate-y-[-2px]"
				>
					<div class="bg-gray-700 h-48 flex items-center justify-center">
						{#if car.image}
							<img
								src={car.image}
								alt="{car.make} {car.model}"
								class="w-full h-full object-cover"
							/>
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

					<div class="p-5">
						<h2 class="text-xl font-semibold">{car.make} {car.model}</h2>
						<div class="flex justify-between items-center mt-2">
							<span class="text-gray-400">{car.year}</span>
							<span class="font-bold text-lg text-white">{formatPrice(car.price)}</span>
						</div>
						<div class="mt-4 pt-4 border-t border-gray-700">
							<a
								href={`/car/${car.id}`}
								class="block w-full bg-purple-600 hover:bg-purple-700 text-white py-2 text-center rounded-lg font-medium transition-colors"
							>
								View Details
							</a>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<div class="bg-yellow-900/30 border border-yellow-700/50 p-6 rounded-xl text-center">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="h-12 w-12 mx-auto text-yellow-600 mb-3"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
				/>
			</svg>
			<p class="text-yellow-300 text-lg font-medium">No cars match your current filters</p>
			<p class="text-yellow-400/70 mt-1">
				Try adjusting your criteria or <button
					on:click={resetFilters}
					class="text-yellow-300 underline hover:text-yellow-200">reset all filters</button
				>
			</p>
		</div>
	{/if}
</div>
