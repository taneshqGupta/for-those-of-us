<script lang="ts">
    import {
        getUserProfile,
        getUserPosts,
        updateProfilePicture,
        updatePost,
        deletePost,
    } from "$lib/api";
    import { page } from "$app/stores";
    import { authStore, logout } from "$lib/auth";
    import { goto } from "$app/navigation";
    import type { Post, UserProfile, Category } from "$lib/types";
    import { CATEGORIES } from "$lib/types";
    import ProfilePicture from "$lib/components/ProfilePicture.svelte";
    import Map from "$lib/components/Map.svelte";
    import {
        MailSvg,
        PinSvg,
        TasksSvg,
        LogoutSvg,
        FilterSvg,
        CrossSvg,
        EditSvg,
        DeleteSvg,
    } from "$lib/components/icons";

    let loading = true;
    let profile: UserProfile | null = null;
    let userPosts: Post[] = [];
    let error = "";
    let profileUpdateLoading = false;

    let currentID: string | null = null;

    let textSearch = "";
    let selectedCategories: Category[] = [];
    let categorySearch = "";
    let postTypeFilter: "both" | "offers" | "requests" = "both";
    let showMobileFilters = false;

    let editingPost: Post | null = null;
    let editDescription = "";
    let editCategories: Category[] = [];
    let editPinCode = "";
    let editCategorySearch = "";
    let editLoading = false;
    let deleteLoading = false;

    let mapCenter: [number, number] = [20.5937, 78.9629]; // Default center of India

    $: isOwnProfile = $authStore.user_id === Number($page.params.userid);

    $: filteredCategories = CATEGORIES.filter((category) =>
        category.toLowerCase().includes(categorySearch.toLowerCase()),
    );

    function toggleCategory(category: Category) {
        if (selectedCategories.includes(category)) {
            selectedCategories = selectedCategories.filter(
                (c) => c !== category,
            );
        } else {
            selectedCategories = [...selectedCategories, category];
        }
        categorySearch = "";
    }

    $: {
        const userid = $page.params.userid;
        if (userid && userid !== currentID) {
            loadProfile(userid);
        }
    }

    $: if (!$authStore.loading && !$authStore.isAuthenticated) {
        goto("/login");
    }

    async function loadProfile(id: string) {
        currentID = id;
        const numericId = Number(id);
        if (isNaN(numericId)) {
            error = "Invalid User-Id in the URL";
            loading = false;
            return;
        }

        try {
            loading = true;
            error = "";
            const [profileData, postsData] = await Promise.all([
                getUserProfile(numericId),
                getUserPosts(numericId),
            ]);
            profile = profileData;
            userPosts = postsData;
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to load profile";
        } finally {
            loading = false;
        }
    }

    async function handleLogout() {
        await logout();
        goto("/login");
    }

    async function handleProfilePictureChange(file: File) {
        if (!$page.params.userid) return;
        try {
            profileUpdateLoading = true;
            error = "";
            const reader = new FileReader();
            reader.onload = async () => {
                try {
                    const base64Data = reader.result as string;
                    await updateProfilePicture(base64Data);
                    await loadProfile($page.params.userid);
                } catch (err) {
                    error =
                        err instanceof Error
                            ? err.message
                            : "Failed to update profile picture";
                } finally {
                    profileUpdateLoading = false;
                }
            };
            reader.readAsDataURL(file);
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to process image";
            profileUpdateLoading = false;
        }
    }

    $: filteredPosts = userPosts.filter((post) => {
        if (postTypeFilter === "offers" && post.post_type !== "offer")
            return false;
        if (postTypeFilter === "requests" && post.post_type !== "request")
            return false;

        if (
            selectedCategories.length > 0 &&
            !selectedCategories.some((cat) => post.categories?.includes(cat))
        )
            return false;

        if (textSearch.trim()) {
            const searchTerm = textSearch.toLowerCase();
            const searchableContent = [
                post.description,
                ...(post.categories || []),
                post.pin_code || "",
                post.user_name || "",
            ]
                .join(" ")
                .toLowerCase();

            if (!searchableContent.includes(searchTerm)) return false;
        }

        return true;
    });

    $: offerCount = userPosts.filter(
        (post) => post.post_type === "offer",
    ).length;
    $: requestCount = userPosts.filter(
        (post) => post.post_type === "request",
    ).length;

    function startEditPost(post: Post) {
        editingPost = post;
        editDescription = post.description;
        editCategories = [...post.categories];
        editPinCode = post.pin_code || "";
        editCategorySearch = "";
    }

    function cancelEdit() {
        editingPost = null;
        editDescription = "";
        editCategories = [];
        editPinCode = "";
        editCategorySearch = "";
    }

    async function saveEdit() {
        if (!editingPost || !editDescription.trim()) return;

        try {
            editLoading = true;
            error = "";

            const updatedPost = {
                ...editingPost,
                description: editDescription.trim(),
                categories: editCategories,
                pin_code: editPinCode.trim() || undefined,
            };

            await updatePost(updatedPost);

            userPosts = userPosts.map((p) =>
                p.id === editingPost?.id ? updatedPost : p,
            );

            cancelEdit();
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to update post";
        } finally {
            editLoading = false;
        }
    }

    async function handleDeletePost(postId: number) {
        if (!confirm("Are you sure you want to delete this post?")) return;

        try {
            deleteLoading = true;
            error = "";

            await deletePost(postId);

            userPosts = userPosts.filter((p) => p.id !== postId);
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to delete post";
        } finally {
            deleteLoading = false;
        }
    }

    function toggleEditCategory(category: Category) {
        if (editCategories.includes(category)) {
            editCategories = editCategories.filter((c) => c !== category);
        } else {
            editCategories = [...editCategories, category];
        }
        editCategorySearch = "";
    }

    $: editFilteredCategories = CATEGORIES.filter((category) =>
        category.toLowerCase().includes(editCategorySearch.toLowerCase()),
    );

    async function getCoordinatesFromPinCode(
        pinCode: string,
    ): Promise<[number, number] | null> {
        try {
            const response = await fetch(
                `https://api.postalpincode.in/pincode/${pinCode}`,
            );
            const data = await response.json();

            if (
                data &&
                data[0] &&
                data[0].Status === "Success" &&
                data[0].PostOffice
            ) {
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

    $: if (profile?.pin_code) {
        getCoordinatesFromPinCode(profile.pin_code).then((coords) => {
            if (coords) {
                mapCenter = coords;
            }
        });
    }
</script>

<svelte:head>
    <title
        >{profile ? `${profile.name}'s Profile` : "User Profile"} - SkillSwap</title
    >
    <meta name="description" content="View a user's SkillSwap profile" />
</svelte:head>

<div class="h-full overflow-y-auto bg-base-100 p-4 gap-4 relative">
    {#if profile?.pin_code}
        <div class="fixed top-16 left-0 right-0 bottom-13 z-0">
            <Map
                posts={filteredPosts}
                center={mapCenter}
                zoom={6}
                height="100%"
                userPinCode={profile.pin_code}
            />
        </div>
    {/if}

    <div class="max-w-4xl mx-auto relative z-10">
        {#if loading}
            <div
                class="skeleton flex h-full w-full justify-center items-center shrink-0 rounded-full"
            ></div>
        {:else if error}
            <div class="alert alert-error">
                <span>{error}</span>
            </div>
        {:else if profile}
            <div class="card bg-base-100 mb-4 p-4">
                <div
                    class="flex flex-row not-md:flex-col items-center justify-center gap-6"
                >
                    <div class="relative flex-shrink-0">
                        <ProfilePicture
                            profilePicture={profile.profile_picture}
                            name={profile.name || "User"}
                            size="xl"
                            editable={isOwnProfile && !profileUpdateLoading}
                            onImageChange={isOwnProfile
                                ? handleProfilePictureChange
                                : null}
                        />
                        {#if profileUpdateLoading}
                            <div
                                class="absolute inset-0 bg-base-200 rounded-full flex items-center justify-center"
                            >
                                <div
                                    class="skeleton flex h-full w-full justify-center items-center shrink-0 rounded-full"
                                ></div>
                            </div>
                        {/if}
                    </div>

                    <div class="text-left not-md:text-center w-full">
                        <div class="join join-vertical">
                            <div
                                class="join-item card-title text-2xl mb-2 flex items-center justify-center md:justify-start"
                            >
                                {profile.name || "User"}
                            </div>
                            {#if isOwnProfile}
                                <div
                                    class="join-item flex items-center justify-center md:justify-start"
                                >
                                    <button
                                        class="btn btn-soft btn-xs flex items-center justify-center mb-2"
                                        on:click={handleLogout}
                                    >
                                        <LogoutSvg /> Log-Out
                                    </button>
                                </div>
                            {/if}
                        </div>
                        <p
                            class="text-sm text-base-content/70 mb-2 text-wrap break-all flex items-center justify-start not-md:justify-center gap-2"
                        >
                            <MailSvg />
                            {profile.email}
                        </p>
                        {#if profile.pin_code}
                            <p
                                class="text-sm text-base-content/70 flex items-center justify-start not-md:justify-center gap-2"
                            >
                                <PinSvg /> Pin Code: {profile.pin_code}
                            </p>
                        {/if}
                        {#if isOwnProfile}
                            <p
                                class="text-xs text-base-content/50 flex items-center justify-start not-md:justify-center mt-2"
                            >
                                Click profile picture to change
                            </p>
                        {/if}
                    </div>
                    <div
                        class="join join-vertical text-right gap-2 not-md:hidden"
                    >
                        <div
                            class="join-item badge badge-xs badge-soft p-4 w-80"
                        >
                            <div class="font-semibold text-xl">
                                {userPosts.length}
                            </div>
                            <div class="font-normal text-xs">Total Posts</div>
                        </div>
                        <div
                            class="join-item badge badge-xs badge-soft p-4 w-80"
                        >
                            <div class="font-semibold text-xl">
                                {offerCount}
                            </div>
                            <div class="font-normal text-xs">Offer Posts</div>
                        </div>
                        <div
                            class="join-item badge badge-xs badge-soft p-4 w-80"
                        >
                            <div class="font-semibold text-xl">
                                {requestCount}
                            </div>
                            <div class="font-normal text-xs">Request Posts</div>
                        </div>
                    </div>
                </div>
                <div
                    class="join join-horizontal flex items-center justify-center text-right gap-0 md:hidden mt-4"
                >
                    <div class="join-item badge badge-xs badge-soft p-4 w-40">
                        <div class="font-semibold text-xl p-4">
                            {userPosts.length}
                        </div>
                        <div class="font-normal text-xs">Posts</div>
                    </div>
                    <div class="join-item badge badge-xs badge-soft p-4 w-40">
                        <div class="font-semibold text-xl">{offerCount}</div>
                        <div class="font-normal text-xs">Offers</div>
                    </div>
                    <div class="join-item badge badge-xs badge-soft p-4 w-40">
                        <div class="font-semibold text-xl">{requestCount}</div>
                        <div class="font-normal text-xs">Requests</div>
                    </div>
                </div>
            </div>

            <div class="card bg-base-100 mb-4 p-4">
                <div class="lg:hidden flex flex-wrap gap-2">
                    <button
                        class="btn btn-soft btn-sm w-full gap-2 flex-1"
                        on:click={() =>
                            (showMobileFilters = !showMobileFilters)}
                    >
                        <FilterSvg />
                        {showMobileFilters ? "Hide" : "Show"} Filters
                        {#if selectedCategories.length > 0 || textSearch.trim() || postTypeFilter !== "both"}
                            <div class="badge badge-xs">
                                {selectedCategories.length +
                                    (textSearch.trim() ? 1 : 0) +
                                    (postTypeFilter !== "both" ? 1 : 0)}
                            </div>
                        {/if}
                    </button>
                    <button
                        class="btn btn-soft btn-sm w-full gap-2 flex-1"
                        on:click={() => {
                            textSearch = "";
                            selectedCategories = [];
                            postTypeFilter = "both";
                            showMobileFilters = false;
                        }}
                        >Clear Filters
                    </button>
                </div>

                <!-- Filter controls -->
                <div
                    class="space-y-4 lg:space-y-3"
                    class:hidden={!showMobileFilters}
                    class:lg:block={true}
                >
                    <!-- Desktop layout: compact horizontal arrangement -->
                    <div class="hidden lg:block space-y-3">
                        <!-- Top row: Search and Clear button -->
                        <div class="flex items-end gap-4">
                            <!-- Text search -->
                            <div class="form-control flex-1 w-full">
                                <label class="label py-1" for="text-search">
                                    <span class="label-text text-xs"
                                        >Search Posts</span
                                    >
                                </label>
                                <input
                                    id="text-search"
                                    class="input input-bordered input-sm px-4 w-full"
                                    placeholder="Search descriptions, categories..."
                                    bind:value={textSearch}
                                />
                            </div>

                            <!-- Clear filters button -->
                            <div class="form-control">
                                <div class="label py-1">
                                    <span class="label-text text-sm opacity-0"
                                        >.</span
                                    >
                                </div>
                                <button
                                    class="btn btn-soft btn-sm"
                                    on:click={() => {
                                        textSearch = "";
                                        selectedCategories = [];
                                        postTypeFilter = "both";
                                        showMobileFilters = false;
                                    }}
                                >
                                    Clear Filters
                                </button>
                            </div>
                        </div>

                        <!-- Bottom row: Categories and Post Type side by side -->
                        <div class="flex gap-4 items-end">
                            <!-- Category filter -->
                            <div class="form-control flex-1">
                                <div class="label py-1">
                                    <span class="label-text text-xs">
                                        Categories ({selectedCategories.length}
                                        selected)
                                    </span>
                                </div>
                                <div class="dropdown w-full">
                                    <div
                                        role="button"
                                        class="btn btn-soft btn-sm w-full justify-start"
                                        tabindex="0"
                                    >
                                        Categories {selectedCategories.length >
                                        0
                                            ? `(${selectedCategories.length})`
                                            : ""}
                                    </div>
                                    <div
                                        class="dropdown-content bg-base-100 rounded-box z-[1] w-80 p-2 shadow mb-2 max-h-80 overflow-y-auto"
                                    >
                                        <!-- Search Input -->
                                        <div class="form-control mb-2">
                                            <input
                                                class="input input-bordered input-xs"
                                                placeholder="Search categories..."
                                                bind:value={categorySearch}
                                            />
                                        </div>

                                        {#if selectedCategories.length > 0}
                                            <div
                                                class="mb-2 p-2 bg-base-200 rounded"
                                            >
                                                <div
                                                    class="flex items-center justify-between mb-1"
                                                >
                                                    <div
                                                        class="text-xs font-semibold"
                                                    >
                                                        Selected:
                                                    </div>
                                                    <button
                                                        class="btn btn-ghost btn-xs"
                                                        on:click={() =>
                                                            (selectedCategories =
                                                                [])}
                                                        >Clear All</button
                                                    >
                                                </div>
                                                <div
                                                    class="flex flex-wrap gap-1"
                                                >
                                                    {#each selectedCategories as category}
                                                        <div
                                                            class="badge badge-primary badge-xs"
                                                        >
                                                            {category}
                                                            <button
                                                                class="ml-1"
                                                                on:click={() =>
                                                                    (selectedCategories =
                                                                        selectedCategories.filter(
                                                                            (
                                                                                c,
                                                                            ) =>
                                                                                c !==
                                                                                category,
                                                                        ))}
                                                                >×</button
                                                            >
                                                        </div>
                                                    {/each}
                                                </div>
                                            </div>
                                        {/if}

                                        <ul
                                            class="menu max-h-40 overflow-y-auto"
                                        >
                                            {#each filteredCategories as category (category)}
                                                <li>
                                                    <label
                                                        class="cursor-pointer flex items-center gap-2 text-xs"
                                                    >
                                                        <input
                                                            type="checkbox"
                                                            class="checkbox checkbox-xs"
                                                            checked={selectedCategories.includes(
                                                                category,
                                                            )}
                                                            on:change={() =>
                                                                toggleCategory(
                                                                    category,
                                                                )}
                                                        />
                                                        <span>{category}</span>
                                                    </label>
                                                </li>
                                            {/each}
                                            {#if filteredCategories.length === 0}
                                                <li>
                                                    <span
                                                        class="text-xs opacity-50"
                                                        >No categories found</span
                                                    >
                                                </li>
                                            {/if}
                                        </ul>
                                    </div>
                                </div>
                            </div>

                            <!-- Post type filter -->
                            <div class="form-control flex-1">
                                <div class="label py-1">
                                    <span class="label-text text-xs"
                                        >Offers/Requests</span
                                    >
                                </div>
                                <div class="dropdown w-full">
                                    <div
                                        role="button"
                                        class="btn btn-soft btn-sm w-full justify-start"
                                        tabindex="0"
                                    >
                                        {postTypeFilter === "both"
                                            ? "Both"
                                            : postTypeFilter === "offers"
                                              ? "Offers Only"
                                              : "Requests Only"}
                                    </div>
                                    <ul
                                        class="dropdown-content menu bg-base-100 rounded-box z-[1] w-full p-2 shadow mb-2"
                                    >
                                        <li>
                                            <label
                                                class="cursor-pointer flex items-center gap-2"
                                            >
                                                <input
                                                    type="radio"
                                                    class="radio radio-sm"
                                                    bind:group={postTypeFilter}
                                                    value="both"
                                                />
                                                <span class="text-sm">Both</span
                                                >
                                            </label>
                                        </li>
                                        <li>
                                            <label
                                                class="cursor-pointer flex items-center gap-2"
                                            >
                                                <input
                                                    type="radio"
                                                    class="radio radio-sm"
                                                    bind:group={postTypeFilter}
                                                    value="offers"
                                                />
                                                <span class="text-sm"
                                                    >Offers Only</span
                                                >
                                            </label>
                                        </li>
                                        <li>
                                            <label
                                                class="cursor-pointer flex items-center gap-2"
                                            >
                                                <input
                                                    type="radio"
                                                    class="radio radio-sm"
                                                    bind:group={postTypeFilter}
                                                    value="requests"
                                                />
                                                <span class="text-sm"
                                                    >Requests Only</span
                                                >
                                            </label>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Mobile layout: vertical stacking (preserved as is) -->
                    <div class="lg:hidden space-y-4">
                        <!-- Text search -->
                        <div class="form-control">
                            <label class="label py-1" for="mobile-text-search">
                                <span class="label-text text-xs"
                                    >Search Posts</span
                                >
                            </label>
                            <input
                                id="mobile-text-search"
                                class="input input-bordered input-sm"
                                placeholder="Search descriptions, categories..."
                                bind:value={textSearch}
                            />
                        </div>

                        <!-- Category filter -->
                        <div class="form-control">
                            <div class="label py-1">
                                <span class="label-text text-xs">
                                    Categories ({selectedCategories.length} selected)
                                </span>
                            </div>
                            <div class="dropdown w-full">
                                <div
                                    role="button"
                                    class="btn btn-soft btn-sm w-full justify-start"
                                    tabindex="0"
                                >
                                    Categories {selectedCategories.length > 0
                                        ? `(${selectedCategories.length})`
                                        : ""}
                                </div>
                                <div
                                    class="dropdown-content bg-base-100 rounded-box z-[1] w-full p-2 shadow mb-2 max-h-80 overflow-y-auto"
                                >
                                    <!-- Search Input -->
                                    <div class="form-control mb-2 w-full">
                                        <input
                                            class="input input-bordered input-xs w-full"
                                            placeholder="Search categories..."
                                            bind:value={categorySearch}
                                        />
                                    </div>

                                    {#if selectedCategories.length > 0}
                                        <div
                                            class="mb-2 bg-base-200 rounded w-full"
                                        >
                                            <div
                                                class="flex items-center justify-between mb-1"
                                            >
                                                <div
                                                    class="text-xs font-semibold"
                                                >
                                                    Selected:
                                                </div>
                                                <button
                                                    class="btn btn-ghost btn-xs"
                                                    on:click={() =>
                                                        (selectedCategories =
                                                            [])}
                                                    >Clear All</button
                                                >
                                            </div>
                                            <div class="flex flex-wrap gap-1">
                                                {#each selectedCategories as category}
                                                    <div
                                                        class="badge badge-primary badge-xs"
                                                    >
                                                        {category}
                                                        <button
                                                            class="ml-1"
                                                            on:click={() =>
                                                                (selectedCategories =
                                                                    selectedCategories.filter(
                                                                        (c) =>
                                                                            c !==
                                                                            category,
                                                                    ))}
                                                            >×</button
                                                        >
                                                    </div>
                                                {/each}
                                            </div>
                                        </div>
                                    {/if}

                                    <ul
                                        class="menu max-h-40 overflow-y-auto w-full"
                                    >
                                        {#each filteredCategories as category (category)}
                                            <li class="w-full">
                                                <label
                                                    class="cursor-pointer flex items-center gap-2 text-xs w-full"
                                                >
                                                    <input
                                                        type="checkbox"
                                                        class="checkbox checkbox-xs"
                                                        checked={selectedCategories.includes(
                                                            category,
                                                        )}
                                                        on:change={() =>
                                                            toggleCategory(
                                                                category,
                                                            )}
                                                    />
                                                    <span class="w-full"
                                                        >{category}</span
                                                    >
                                                </label>
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

                        <!-- Post type filter -->
                        <div class="form-control">
                            <div class="label py-1">
                                <span class="label-text text-xs"
                                    >Offer/Request</span
                                >
                            </div>
                            <div class="dropdown w-full">
                                <div
                                    role="button"
                                    class="btn btn-soft btn-sm w-full justify-start"
                                    tabindex="0"
                                >
                                    {postTypeFilter === "both"
                                        ? "Both"
                                        : postTypeFilter === "offers"
                                          ? "Offers Only"
                                          : "Requests Only"}
                                </div>
                                <ul
                                    class="dropdown-content menu bg-base-100 rounded-box z-[1] w-full p-2 shadow mb-2"
                                >
                                    <li>
                                        <label
                                            class="cursor-pointer flex items-center gap-2"
                                        >
                                            <input
                                                type="radio"
                                                class="radio radio-sm"
                                                bind:group={postTypeFilter}
                                                value="both"
                                            />
                                            <span class="text-sm">Both</span>
                                        </label>
                                    </li>
                                    <li>
                                        <label
                                            class="cursor-pointer flex items-center gap-2"
                                        >
                                            <input
                                                type="radio"
                                                class="radio radio-sm"
                                                bind:group={postTypeFilter}
                                                value="offers"
                                            />
                                            <span class="text-sm"
                                                >Offers Only</span
                                            >
                                        </label>
                                    </li>
                                    <li>
                                        <label
                                            class="cursor-pointer flex items-center gap-2"
                                        >
                                            <input
                                                type="radio"
                                                class="radio radio-sm"
                                                bind:group={postTypeFilter}
                                                value="requests"
                                            />
                                            <span class="text-sm"
                                                >Requests Only</span
                                            >
                                        </label>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="card bg-base-100 p-4">
                <h2 class="card-title text-xl mb-4">
                    <TasksSvg />
                    {isOwnProfile ? "Your Posts" : `${profile.name}'s Posts`}
                    {#if filteredPosts.length !== userPosts.length}
                        <span class="text-sm text-base-content/60 font-normal">
                            ({filteredPosts.length} of {userPosts.length} shown)
                        </span>
                    {/if}
                </h2>

                {#if filteredPosts.length > 0}
                    <div class="space-y-4">
                        {#each filteredPosts as post}
                            <div class="card card-compact bg-base-200">
                                <div class="card-body">
                                    {#if editingPost?.id === post.id}
                                        <div class="w-full">
                                            <div
                                                class="join join-vertical w-full"
                                            >
                                                <div class="join-item w-full">
                                                    <div
                                                        class="join join-horizontal w-full"
                                                    >
                                                        <div
                                                            class="join-item w-full form-control mr-7"
                                                        >
                                                            <fieldset
                                                                class="fieldset"
                                                            >
                                                                <legend
                                                                    class="fieldset-legend"
                                                                    >Description</legend
                                                                >
                                                                <textarea
                                                                    class="input textarea h-39.5 w-full text-wrap"
                                                                    placeholder="Edit your post description .."
                                                                    bind:value={
                                                                        editDescription
                                                                    }
                                                                    required
                                                                ></textarea>
                                                            </fieldset>
                                                        </div>
                                                        <div
                                                            class="join-item w-40"
                                                        >
                                                            <div
                                                                class="join join-vertical"
                                                            >
                                                                <fieldset
                                                                    class="fieldset"
                                                                >
                                                                    <legend
                                                                        class="fieldset-legend"
                                                                        >Pin
                                                                        Code</legend
                                                                    >
                                                                    <input
                                                                        id="edit-pincode-{post.id}"
                                                                        class="input input-bordered w-full"
                                                                        type="text"
                                                                        placeholder="Pin Code (optional)"
                                                                        bind:value={
                                                                            editPinCode
                                                                        }
                                                                        maxlength="10"
                                                                    />
                                                                </fieldset>
                                                                <fieldset
                                                                    class="join-item fieldset"
                                                                >
                                                                    <legend
                                                                        class="fieldset-legend"
                                                                    ></legend>
                                                                    <button
                                                                        class="btn btn-success btn-block"
                                                                        on:click={saveEdit}
                                                                        disabled={editLoading ||
                                                                            !editDescription.trim() ||
                                                                            editCategories.length ===
                                                                                0}
                                                                    >
                                                                        {#if editLoading}
                                                                            <span
                                                                                class="loading loading-infinity loading-xs"

                                                                            ></span>
                                                                        {:else}
                                                                            Save
                                                                        {/if}
                                                                    </button>
                                                                </fieldset>
                                                                <fieldset
                                                                    class="join-item fieldset"
                                                                >
                                                                    <legend
                                                                        class="fieldset-legend"
                                                                    ></legend>
                                                                    <button
                                                                        class="btn btn-error btn-block"
                                                                        on:click={cancelEdit}
                                                                        disabled={editLoading}
                                                                    >
                                                                        Cancel
                                                                    </button>
                                                                </fieldset>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>

                                                <div class="join-item w-full">
                                                    <div
                                                        class="join join-vertical w-full"
                                                    >
                                                        <div
                                                            class="join-item form-control"
                                                        >
                                                            <fieldset
                                                                class="fieldset"
                                                            >
                                                                <legend
                                                                    class="fieldset-legend"
                                                                    >Categories
                                                                    ({editCategories.length}
                                                                    selected)</legend
                                                                >
                                                            </fieldset>
                                                        </div>

                                                        {#if editCategories.length > 0}
                                                            <div
                                                                class="join-item flex flex-wrap gap-2 mb-3 max-h-32 overflow-y-auto rounded"
                                                            >
                                                                {#each editCategories as category}
                                                                    <div
                                                                        class="join gap-1"
                                                                    >
                                                                        <div
                                                                            class="join-item badge badge-primary badge-lg textarea-md"
                                                                        >
                                                                            {category}
                                                                        </div>
                                                                        <div
                                                                            class="pt-0.25"
                                                                        >
                                                                            <button
                                                                                class="join-item btn btn-ghost btn-circle btn-xs flex items-center justify-center"
                                                                                on:click={() =>
                                                                                    toggleEditCategory(
                                                                                        category,
                                                                                    )}
                                                                            >
                                                                                <div
                                                                                    class="flex items-center justify-center"
                                                                                >
                                                                                    <CrossSvg
                                                                                    />
                                                                                </div>
                                                                            </button>
                                                                        </div>
                                                                    </div>
                                                                {/each}
                                                            </div>
                                                        {/if}

                                                        <div
                                                            class="join-item dropdown dropdown-bottom w-full"
                                                        >
                                                            <div
                                                                role="button"
                                                                class="btn btn-outline btn-block justify-start"
                                                                tabindex="0"
                                                            >
                                                                Select
                                                                Categories
                                                            </div>
                                                            <div
                                                                class="dropdown-content bg-base-100 rounded-box z-[1] p-2 shadow w-full max-h-60 overflow-y-auto"
                                                            >
                                                                <div
                                                                    class="form-control mb-2"
                                                                >
                                                                    <input
                                                                        class="input input-bordered input-sm"
                                                                        placeholder="Search categories..."
                                                                        bind:value={
                                                                            editCategorySearch
                                                                        }
                                                                    />
                                                                </div>
                                                                <ul
                                                                    class="menu w-full"
                                                                >
                                                                    {#each editFilteredCategories as category}
                                                                        <li>
                                                                            <button
                                                                                type="button"
                                                                                class="text-left text-sm {editCategories.includes(
                                                                                    category,
                                                                                )
                                                                                    ? 'active'
                                                                                    : ''}"
                                                                                on:click={() =>
                                                                                    toggleEditCategory(
                                                                                        category,
                                                                                    )}
                                                                            >
                                                                                <span
                                                                                    class="flex items-center gap-2"
                                                                                    ><input
                                                                                        type="checkbox"
                                                                                        checked={editCategories.includes(
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
                                                                        <li
                                                                            class="w-full"
                                                                        >
                                                                            <span
                                                                                class="text-xs opacity-50 w-full"
                                                                                >No
                                                                                categories
                                                                                found</span
                                                                            >
                                                                        </li>
                                                                    {/if}
                                                                </ul>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    {:else}
                                        <!-- Display Mode -->
                                        <div
                                            class="flex items-center justify-between text-xs text-base-content/60 mb-2"
                                        >
                                            <div class="flex gap-2">
                                                {#if post.post_type == "offer"}
                                                    <span
                                                        class="badge badge-primary font-black"
                                                        >Offer</span
                                                    >
                                                {:else}
                                                    <span
                                                        class="badge badge-accent font-black"
                                                        >Request</span
                                                    >
                                                {/if}
                                                {#if post.pin_code}
                                                    <span
                                                        class="badge badge-outline flex items-center gap-1"
                                                    >
                                                        <PinSvg />
                                                        {post.pin_code}
                                                    </span>
                                                {/if}
                                            </div>

                                            {#if isOwnProfile}
                                                <div class="flex gap-1">
                                                    <button
                                                        class="btn btn-ghost btn-circle"
                                                        on:click={() =>
                                                            startEditPost(post)}
                                                        disabled={editingPost !==
                                                            null ||
                                                            deleteLoading}
                                                        title="Edit post"
                                                    >
                                                        <EditSvg />
                                                    </button>
                                                    <button
                                                        class="btn btn-ghost btn-circle"
                                                        on:click={() =>
                                                            handleDeletePost(
                                                                post.id,
                                                            )}
                                                        disabled={editingPost !==
                                                            null ||
                                                            deleteLoading}
                                                        title="Delete post"
                                                    >
                                                        {#if deleteLoading}
                                                            <span
                                                                class="loading loading-infinity loading-xs"
                                                            ></span>
                                                        {:else}
                                                            <DeleteSvg />
                                                        {/if}
                                                    </button>
                                                </div>
                                            {/if}
                                        </div>

                                        <p class="mb-3">
                                            {post.description}
                                        </p>

                                        {#if post.categories && post.categories.length > 0}
                                            <div class="flex flex-wrap gap-1">
                                                {#each post.categories as category}
                                                    <div
                                                        class="badge badge-soft badge-md"
                                                    >
                                                        {category}
                                                    </div>
                                                {/each}
                                            </div>
                                        {/if}
                                    {/if}
                                </div>
                            </div>
                        {/each}
                    </div>
                {:else if userPosts.length > 0}
                    <div class="text-center p-8 text-base-content/60">
                        <h3 class="font-bold mb-2">
                            No Posts Match Your Filters
                        </h3>
                        <p>Try adjusting your filters to see more posts.</p>
                        <button
                            class="btn btn-soft btn-sm mt-2"
                            on:click={() => {
                                textSearch = "";
                                selectedCategories = [];
                                postTypeFilter = "both";
                                showMobileFilters = false;
                            }}
                        >
                            Clear Filters
                        </button>
                    </div>
                {:else}
                    <div class="text-center p-8 text-base-content/60">
                        <h3 class="font-bold mb-2">No Posts Yet</h3>
                        <p>
                            {isOwnProfile
                                ? "Start sharing your skills or requesting help!"
                                : "This user has not made any posts yet."}
                        </p>
                    </div>
                {/if}
            </div>
        {/if}
    </div>
</div>
