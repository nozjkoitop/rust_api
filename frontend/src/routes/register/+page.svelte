<script>
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';

	let loading = false;
	let error = '';
</script>

<div class="flex justify-center items-center min-h-[80vh]">
	<div class="w-full max-w-md p-8 space-y-8 bg-gray-800 rounded-xl shadow-lg">
		<div class="text-center">
			<h1 class="text-3xl font-bold">Register</h1>
			<p class="mt-2 text-gray-400">Create a new account</p>
		</div>

		{#if error}
			<div class="p-4 bg-red-900/50 border border-red-700 rounded-lg text-red-200">
				{error}
			</div>
		{/if}

		<form
			method="POST"
			action="?/register"
			use:enhance={() => {
				loading = true;
				error = '';

				return async ({ result }) => {
					loading = false;

					if (result.type === 'failure') {
						error = result.data?.message || 'Registration failed. Please try again.';
					} else if (result.type === 'redirect') {
						goto(result.location);
					}
				};
			}}
			class="space-y-6"
		>
			<div>
				<label for="username" class="block text-sm font-medium text-gray-300">Username</label>
				<input
					id="username"
					name="username"
					type="text"
					required
					class="mt-1 block w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				/>
			</div>

			<div>
				<label for="email" class="block text-sm font-medium text-gray-300">Email</label>
				<input
					id="email"
					name="email"
					type="email"
					required
					class="mt-1 block w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				/>
			</div>

			<div>
				<label for="password" class="block text-sm font-medium text-gray-300">Password</label>
				<input
					id="password"
					name="password"
					type="password"
					required
					class="mt-1 block w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				/>
			</div>

			<div>
				<label for="confirmPassword" class="block text-sm font-medium text-gray-300"
					>Confirm Password</label
				>
				<input
					id="confirmPassword"
					name="confirmPassword"
					type="password"
					required
					class="mt-1 block w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
				/>
			</div>

      <div>
        By creating a CarTrader account, you agree to the CarTrader Terms of Service.
      </div>

			<div>
				<button
					type="submit"
					disabled={loading}
					class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-purple-600 hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{loading ? 'Creating account...' : 'Create account'}
				</button>
			</div>
		</form>

		<div class="text-center mt-4">
			<p class="text-sm text-gray-400">
				Already have an account?
				<a href="/login" class="font-medium text-purple-400 hover:text-purple-300">Login</a>
			</p>
		</div>
	</div>
</div>
