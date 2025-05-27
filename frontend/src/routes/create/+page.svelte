<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';

	let loading = false;
	let error = '';
	let success = '';

	const currentYear = new Date().getFullYear();
	const years = Array.from({ length: 50 }, (_, i) => currentYear - i);

	let selectedFiles: any[] = [];
	let imagePreviewUrls: any[] = [];

	function handleFileSelect(event: { target: { files: Iterable<unknown> | ArrayLike<unknown> } }) {
		const files = Array.from(event.target.files);
		selectedFiles = [...selectedFiles, ...files];

		//@ts-ignore
		files.forEach((file: Blob) => {
			const reader = new FileReader();
			reader.onload = (e) => {
				imagePreviewUrls = [...imagePreviewUrls, e.target?.result as string];
			};
			reader.readAsDataURL(file);
		});
	}

	function removeImage(index: number) {
		selectedFiles = selectedFiles.filter((_, i) => i !== index);
		imagePreviewUrls = imagePreviewUrls.filter((_, i) => i !== index);
	}
</script>

<div class="max-w-4xl mx-auto">
	<div class="mb-8">
		<h1 class="text-3xl font-bold">Add New Car</h1>
		<p class="text-gray-400 mt-2">Enter the details of the car you want to add to inventory</p>
	</div>
	{#if error}
		<div class="p-4 mb-6 bg-red-900/50 border border-red-700 rounded-lg text-red-200">
			{error}
		</div>
	{/if}

	{#if success}
		<div class="p-4 mb-6 bg-green-900/50 border border-green-700 rounded-lg text-green-200">
			{success}
		</div>
	{/if}

	<div class="bg-gray-800 rounded-xl p-6 shadow-lg">
		<form
			method="POST"
			action="?/createCar"
			enctype="multipart/form-data"
			use:enhance={() => {
				loading = true;
				error = '';
				success = '';

				return async ({ result }) => {
					loading = false;

					if (result.type === 'failure') {
						error = result.data?.message || 'Failed to create car. Please try again.';
					} else if (result.type === 'success') {
						success = 'Car added successfully!';
					}
				};
			}}
			class="space-y-6"
		>
			<input type="hidden" name="files" value={JSON.stringify(selectedFiles)} />
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div>
					<label for="make" class="block text-sm font-medium text-gray-300 mb-1">Make</label>
					<input
						id="make"
						name="make"
						type="text"
						required
						placeholder="e.g. Toyota"
						class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:ring-2 focus:ring-purple-500 focus:border-transparent"
					/>
				</div>

				<div>
					<label for="model" class="block text-sm font-medium text-gray-300 mb-1">Model</label>
					<input
						id="model"
						name="model"
						type="text"
						required
						placeholder="e.g. Camry"
						class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:ring-2 focus:ring-purple-500 focus:border-transparent"
					/>
				</div>

				<div>
					<label for="year" class="block text-sm font-medium text-gray-300 mb-1">Year</label>
					<select
						id="year"
						name="year"
						required
						class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:ring-2 focus:ring-purple-500 focus:border-transparent"
					>
						<option value="" disabled selected>Select year</option>
						{#each years as year}
							<option value={year}>{year}</option>
						{/each}
					</select>
				</div>

				<div>
					<label for="price" class="block text-sm font-medium text-gray-300 mb-1">Price</label>
					<div class="relative">
						<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
							<span class="text-gray-400">$</span>
						</div>
						<input
							id="price"
							name="price"
							type="number"
							step="0.01"
							min="0"
							required
							placeholder="0.00"
							class="w-full pl-8 pr-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:ring-2 focus:ring-purple-500 focus:border-transparent"
						/>
					</div>
				</div>
			</div>

			<div>
				<label class="block text-sm font-medium text-gray-300 mb-1">Car Images</label>
				<div
					class="mt-1 flex justify-center px-6 pt-5 pb-6 border-2 border-gray-600 border-dashed rounded-md hover:border-gray-500 transition-colors"
				>
					<div class="space-y-1 text-center">
						<svg
							class="mx-auto h-12 w-12 text-gray-400"
							stroke="currentColor"
							fill="none"
							viewBox="0 0 48 48"
							aria-hidden="true"
						>
							<path
								d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							/>
						</svg>
						<div class="flex text-sm text-gray-400">
							<label
								for="file-upload"
								class="relative cursor-pointer bg-gray-700 rounded-md font-medium text-purple-400 hover:text-purple-300 focus-within:outline-none focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-purple-500 px-3 py-2"
							>
								<span>Upload images</span>
								<input
									id="file-upload"
									name="file-upload"
									type="file"
									class="sr-only"
									multiple
									accept="image/*"
									on:change={handleFileSelect}
								/>
							</label>
							<p class="pl-1 pt-2">or drag and drop</p>
						</div>
						<p class="text-xs text-gray-400">PNG, JPG, GIF up to 10MB</p>
					</div>
				</div>
			</div>

			{#if imagePreviewUrls.length > 0}
				<div>
					<h3 class="text-sm font-medium text-gray-300 mb-2">
						Selected Images ({imagePreviewUrls.length})
					</h3>
					<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
						{#each imagePreviewUrls as url, index}
							<div class="relative group">
								<img
									src={url || '/placeholder.svg'}
									alt="Preview"
									class="h-24 w-full object-cover rounded-md"
								/>
								<button
									type="button"
									class="absolute top-1 right-1 bg-red-600 text-white rounded-full p-1 opacity-0 group-hover:opacity-100 transition-opacity"
									on:click={() => removeImage(index)}
								>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										class="h-4 w-4"
										viewBox="0 0 20 20"
										fill="currentColor"
									>
										<path
											fill-rule="evenodd"
											d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
											clip-rule="evenodd"
										/>
									</svg>
								</button>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<div class="flex items-center justify-end space-x-4 pt-4">
				<button
					type="button"
					on:click={() => goto('/')}
					class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors"
				>
					Cancel
				</button>
				<button
					type="submit"
					disabled={loading}
					class="px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{loading ? 'Adding...' : 'Add Car'}
				</button>
			</div>
		</form>
	</div>
</div>
