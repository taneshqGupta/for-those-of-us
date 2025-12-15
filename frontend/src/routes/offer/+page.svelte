<script lang="ts">
	import type { PostType, Category } from "$lib/types";
	import { createPost, getMyProfile } from "$lib/api";
	import { authStore } from "$lib/auth";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";
	import { CATEGORIES } from "$lib/types";
	import { CrossSvg } from "$lib/components/icons";
	import Map from "$lib/components/Map.svelte";
	let newPostDescription = "";
	let newPostCategories: Category[] = [];
	const newPostType: PostType = "offer";
	let newPinCode = "";
	let userDefaultPinCode = "";
	let loading = false;
	let success = "";
	let error = "";
	let categorySearch = "";

	let mapCenter: [number, number] = [20.5937, 78.9629]; // Default center of India

	$: filteredCategories = CATEGORIES.filter((cat) =>
		cat.toLowerCase().includes(categorySearch.toLowerCase()),
	);

	$: if (!$authStore.loading && !$authStore.isAuthenticated) {
		goto("/login");
	}

	onMount(async () => {
		if ($authStore.isAuthenticated) {
			try {
				const userProfile = await getMyProfile();
				userDefaultPinCode = userProfile.pin_code || "";
				newPinCode = userDefaultPinCode;
			} catch (error) {
				console.error("Error loading user profile:", error);
			}
		}
	});

	async function getCoordinatesFromPinCode(pinCode: string): Promise<[number, number] | null> {
		try {
			const response = await fetch(
				`https://api.postalpincode.in/pincode/${pinCode}`
			);
			const data = await response.json();
			
			if (data && data[0] && data[0].Status === "Success" && data[0].PostOffice) {
				const postOffice = data[0].PostOffice[0];
				const lat = parseFloat(postOffice.Latitude) || 20.5937;
				const lng = parseFloat(postOffice.Longitude) || 78.9629;
				return [lat, lng];
			}
		} catch (error) {
			console.error("Error fetching coordinates for pin code:", error);
		}
		return null;
	}

	$: if (newPinCode || userDefaultPinCode) {
		const pinCode = newPinCode || userDefaultPinCode;
		if (pinCode) {
			getCoordinatesFromPinCode(pinCode).then(coords => {
				if (coords) {
					mapCenter = coords;
				}
			});
		}
	}

	async function handleCreatePost() {
		if (!newPostDescription.trim()) {
			error = "Please describe your skill";
			return;
		}

		if (newPostCategories.length === 0) {
			error = "Please select at least one category";
			return;
		}

		const description = newPostDescription.trim();
		const categories = newPostCategories;
		const pinCode = newPinCode.trim() || userDefaultPinCode;

		try {
			loading = true;
			error = "";

			await createPost(description, categories, newPostType, pinCode);

			success = "Skill offer posted successfully!";

			newPostDescription = "";
			newPostCategories = [];
			newPinCode = userDefaultPinCode;

			setTimeout(() => goto("/"), 1500);
		} catch (err) {
			error =
				err instanceof Error ? err.message : "Failed to create post";
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Offer Skill - SkillSwap</title>
	<meta name="description" content="Share your skills with the community" />
</svelte:head>

{#if $authStore.isAuthenticated}
	<!-- Map Background -->
	{#if newPinCode || userDefaultPinCode}
		<div class="fixed top-16 left-0 right-0 bottom-13 z-0">
			<Map 
				posts={[]} 
				center={mapCenter}
				zoom={6}
				height="100%"
				userPinCode={newPinCode || userDefaultPinCode}
			/>
		</div>
	{/if}

	<div
		class="min-h-full overflow-y-auto p-4 flex items-center justify-center relative z-10 pointer-events-none"
	>
		<div
			class="card card-border w-full max-w-md shadow-2xl bg-base-100 my-4 pointer-events-auto"
		>
			<div class="card-body">
				<div class="text-center mb-8">
					<h2
						class="card-title text-2xl font-bold text-base-content justify-center"
					>
						Offer a Skill
					</h2>
					<p class="text-base-content/70 text-sm">
						Share something you're good at to people around you.
					</p>
				</div>

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

				<form class="space-y-6">
					<div class="form-control">
						<fieldset class="fieldset">
							<legend class="fieldset-legend"
								>What skill can you share?</legend
							>
							<textarea
								class="input textarea h-24 w-full"
								placeholder="e.g., I can teach maths, design websites .."
								bind:value={newPostDescription}
								required
							></textarea>
						</fieldset>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-2 gap-1 w-full">
						<div class="form-control pb-4">
							<div class="label text-xs font-black mb-2">
								<span class="label-text-alt">Categories</span>
								{#if newPostCategories.length > 0}
									<span class="text-xs text-base-content/60"
										>({newPostCategories.length} selected)</span
									>
								{/if}
							</div>

							{#if newPostCategories.length > 0}
								<div
									class="flex flex-wrap gap-2 mb-2 max-h-32 overflow-y-auto rounded"
								>
									{#each newPostCategories as category}
										<div class="join">
											<div
												class="badge badge-lg badge-soft textarea-xs"
											>
												{category}
											</div>
											<button
												type="button"
												class="btn btn-ghost btn-circle btn-xs ml-1 mt-0.5"
												on:click={() => {
													newPostCategories =
														newPostCategories.filter(
															(c) =>
																c !== category,
														);
												}}
											>
												<CrossSvg />
											</button>
										</div>
									{/each}
								</div>
							{/if}

							<div class="dropdown dropdown-bottom w-full">
								<div
									role="button"
									class="btn btn-soft btn-block justify-start"
									tabindex="0"
								>
									Select Categories
								</div>
								<div
									class="dropdown-content bg-base-100 rounded-box z-[1] p-2 shadow w-full max-h-60 overflow-y-auto"
								>
									<div class="form-control mb-2">
										<input
											class="input input-bordered input-sm"
											placeholder="Search categories..."
											bind:value={categorySearch}
										/>
									</div>

									<ul class="menu w-full">
										{#each filteredCategories as category}
											<li>
												<button
													type="button"
													class="text-left text-sm {newPostCategories.includes(
														category,
													)
														? 'active'
														: ''}"
													on:click={() => {
														if (
															newPostCategories.includes(
																category,
															)
														) {
															newPostCategories =
																newPostCategories.filter(
																	(c) =>
																		c !==
																		category,
																);
														} else {
															newPostCategories =
																[
																	...newPostCategories,
																	category,
																];
														}
													}}
												>
													<span
														class="flex items-center gap-2"
													>
														<input
															type="checkbox"
															checked={newPostCategories.includes(
																category,
															)}
															class="checkbox checkbox-xs"
															readonly
														/>
														{category}
													</span>
												</button>
											</li>
										{/each}
										{#if filteredCategories.length === 0}
											<li class="w-full">
												<span
													class="text-xs opacity-50 w-full"
													>No categories found</span
												>
											</li>
										{/if}
									</ul>
								</div>
							</div>
						</div>

						<div class="form-control">
							<div class="label text-xs font-black mb-2">
								<span class="label-text-alt">
									{userDefaultPinCode
										? `Pin Code [Your Default: ${userDefaultPinCode}]`
										: "Pin Code"}
								</span>
							</div>
							<input
								id="skill-pincode"
								name="pincode"
								class="input input-bordered w-full"
								type="text"
								placeholder={userDefaultPinCode ||
									"e.g., 110001"}
								bind:value={newPinCode}
								maxlength="10"
								title="Enter your pin code (optional)"
							/>
						</div>
					</div>

					<div class="flex flex-col gap-3 pt-4">
						<button
							class="btn btn-soft btn-block"
							on:click={handleCreatePost}
						>
							{#if loading}
								<span
									class="loading loading-infinity loading-sm"
								></span>
							{:else}
								Offer Skill
							{/if}
						</button>
						<button
							type="button"
							class="btn btn-soft btn-block"
							on:click={() => goto("/")}>Cancel</button
						>
					</div>
				</form>
			</div>
		</div>
	</div>
{/if}
