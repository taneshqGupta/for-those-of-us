<script lang="ts">
	import { login, register, getCommunityPosts } from "$lib/api";
	import { goto } from "$app/navigation";
	import { setAuthenticated, authStore } from "$lib/auth";
	import { onMount } from "svelte";
	import type { Post } from "$lib/types";
	import ProfilePicture from "$lib/components/ProfilePicture.svelte";

	let isLogin = true;
	let email = "";
	let password = "";
	let name = "";
	let pinCode = "";
	let profilePictureFile: File | null = null;
	let profilePicturePreview: string | null = null;
	let loading = false;
	let error = "";
	let success = "";
	let communityStats = { offers: 0, requests: 0, total: 0 };

	onMount(async () => {
		// Load community stats for preview
		try {
			const communityPosts = await getCommunityPosts();
			communityStats.total = communityPosts.length;
			communityStats.offers = communityPosts.filter(
				(p) => p.post_type === "offer",
			).length;
			communityStats.requests = communityPosts.filter(
				(p) => p.post_type === "request",
			).length;
		} catch (err) {
			console.log("Could not load community stats for preview");
		}
	});

	// Redirect to home if already authenticated
	$: if ($authStore.isAuthenticated) {
		goto("/");
	}

	async function handleSubmit() {
		if (!email || !password) {
			error = "Please fill in all fields";
			return;
		}

		if (!isLogin && !name) {
			error = "Please enter your name";
			return;
		}

		if (password.length < 6) {
			error = "Password must be at least 6 characters long";
			return;
		}

		loading = true;
		error = "";
		success = "";

		try {
			if (isLogin) {
				const response = await login(email, password);
				if (response.success && response.user_id) {
					success = "Login successful! Redirecting...";
					setAuthenticated(response.user_id);
					setTimeout(() => goto("/"), 1000);
				} else {
					error = response.message;
				}
			} else {
				// Convert profile picture to base64 if provided
				let profilePictureData: string | undefined = undefined;
				if (profilePictureFile) {
					try {
						profilePictureData =
							await fileToBase64(profilePictureFile);
					} catch (e) {
						error = "Failed to process profile picture";
						loading = false;
						return;
					}
				}

				const response = await register(
					email,
					password,
					name,
					pinCode,
					profilePictureData,
				);
				if (response.success && response.user_id) {
					success = "Registration successful! Redirecting...";
					setAuthenticated(response.user_id);
					setTimeout(() => goto("/"), 1000);
				} else {
					error = response.message;
				}
			}
		} catch (e) {
			error = e instanceof Error ? e.message : "An error occurred";
		} finally {
			loading = false;
		}
	}

	function toggleMode() {
		isLogin = !isLogin;
		error = "";
		success = "";
		name = ""; // Clear name when switching modes
		pinCode = ""; // Clear pin code when switching modes
		profilePictureFile = null; // Clear profile picture when switching modes
		profilePicturePreview = null;
	}

	function handleProfilePictureChange(file: File) {
		profilePictureFile = file;

		// Create preview URL
		const reader = new FileReader();
		reader.onload = (e) => {
			profilePicturePreview = e.target?.result as string;
		};
		reader.readAsDataURL(file);
	}

	// Convert file to base64 for API
	function fileToBase64(file: File): Promise<string> {
		return new Promise((resolve, reject) => {
			const reader = new FileReader();
			reader.onload = () => resolve(reader.result as string);
			reader.onerror = reject;
			reader.readAsDataURL(file);
		});
	}
</script>

<svelte:head>
	<title>SkillSwap - Login</title>
	<meta
		name="description"
		content="Join SkillSwap to discover and share skills in your neighborhood"
	/>
</svelte:head>

<div class="h-full flex items-center justify-center">
	<div class="card card-border w-full max-w-md shadow-2xl bg-base-100">
		<div class="card-body">
			<div class="text-center mb-6">
				<h2
					class="card-title text-2xl font-bold text-base-content justify-center"
				>
					Welcome to Skill-Swap
				</h2>
				<p class="text-base-content/70 text-sm">
					Learn, Teach, Socialise
				</p>
			</div>

			<form on:submit|preventDefault={handleSubmit} class="space-y-4">
				{#if !isLogin}
					<div class="form-control">
						<label class="label" for="name">
							<span class="label-text">Full Name</span>
						</label>
						<input
							id="name"
							name="name"
							type="text"
							class="input input-bordered w-full"
							placeholder="Enter your full name"
							bind:value={name}
							disabled={loading}
							required
						/>
					</div>

					<!-- Profile Picture Upload -->
					<div class="form-control">
						<div class="label">
							<span class="label-text"
								>Profile Picture (Optional)</span
							>
						</div>
						<div class="flex flex-col items-center gap-3">
							<ProfilePicture
								profilePicture={profilePicturePreview}
								{name}
								size="lg"
								editable={!loading}
								onImageChange={handleProfilePictureChange}
							/>
							<p
								class="text-xs text-base-content/60 text-center max-w-xs"
							>
								Click to upload a profile picture or we'll use a
								default one
							</p>
						</div>
					</div>

					<div class="form-control">
						<label class="label" for="pinCode">
							<span class="label-text">Pin Code (Optional)</span>
						</label>
						<input
							id="pinCode"
							name="pinCode"
							type="text"
							class="input input-bordered w-full"
							placeholder="e.g., 110001"
							bind:value={pinCode}
							maxlength="10"
							disabled={loading}
							title="Enter a valid 6-digit pin code"
						/>
						<div class="text-xs text-base-content/60 mt-1">
							This will be your default location for posts
						</div>
					</div>
				{/if}

				<div class="form-control">
					<label class="label" for="email">
						<span class="label-text">Email Address</span>
					</label>
					<input
						id="email"
						name="email"
						type="email"
						class="input input-bordered w-full"
						placeholder="Enter your email"
						bind:value={email}
						disabled={loading}
						required
					/>
				</div>

				<div class="form-control">
					<label class="label" for="password">
						<span class="label-text">Password</span>
					</label>
					<input
						id="password"
						name="password"
						type="password"
						class="input input-bordered w-full"
						placeholder="Enter your password"
						bind:value={password}
						disabled={loading}
						required
						minlength="6"
					/>
					{#if !isLogin}
						<div class="text-xs text-base-content/60 mt-1">
							Minimum 6 characters
						</div>
					{/if}
				</div>

				<!-- Error/Success Messages -->
				{#if error}
					<div class="alert alert-error">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="stroke-current flex-shrink-0 h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
						<span>{error}</span>
					</div>
				{/if}

				{#if success}
					<div class="alert alert-success">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="stroke-current flex-shrink-0 h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
						<span>{success}</span>
					</div>
				{/if}

				<!-- Submit Button -->
				<button
					type="submit"
					class="btn btn-primary w-full"
					disabled={loading}
				>
					{#if loading}
						<span class="loading loading-infinity loading-xs"></span>
						{isLogin ? "Signing In..." : "Creating Account..."}
					{:else}
						{isLogin ? "Sign In" : "Create Account"}
					{/if}
				</button>
			</form>

			<!-- Toggle Mode -->
			<div class="divider">OR</div>
			<button
				class="btn btn-ghost w-full"
				on:click={toggleMode}
				disabled={loading}
			>
				{isLogin
					? "Don't have an account? Sign Up"
					: "Already have an account? Sign In"}
			</button>
		</div>
	</div>
</div>
