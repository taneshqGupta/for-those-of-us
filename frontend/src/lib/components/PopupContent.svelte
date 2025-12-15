<script lang="ts">
    import type { Post } from "$lib/types";
    import { goto } from "$app/navigation";

    export let pinCode: string;
    export let posts: Post[];

    $: offerCount = posts.filter((p) => p.post_type === "offer").length;
    $: requestCount = posts.filter((p) => p.post_type === "request").length;

    function goToProfile(userId: number) {
        goto(`/profile/${userId}`);
    }
</script>

<div class="card card-compact bg-base-200 w-80 max-w-sm p-1.5">
    <!-- <div class="card-body"> -->
    <div class="flex items-center justify-between mb-3">
        <div class="font-medium text-sm">
            Pin Code: {pinCode}
        </div>
        <div class="font-extrabold text-md">
            {offerCount} offers, {requestCount} requests
        </div>
    </div>

    <div class="space-y-3 max-h-60 overflow-y-auto">
        {#each posts as post}
            <div class="card card-compact bg-base-100 border border-base-300">
                <div class="card-body">
                    <div class="flex items-center justify-between mb-2">
                        {#if post.post_type == "offer"}
                            <div class="badge badge-soft">
                                Offer
                            </div>
                        {:else}
                            <div class="badge badge-soft">
                                Request
                            </div>
                        {/if}
                    </div>

                    <p class="mb-2">{post.description}</p>

                    {#if post.categories && post.categories.length > 0}
                        <div class="flex flex-wrap gap-1 mb-2">
                            {#each post.categories as category}
                                <div class="badge badge-soft badge-md">
                                    {category}
                                </div>
                            {/each}
                        </div>
                    {/if}

                    {#if post.user_name}
                        <div class="card-actions justify-center">
                            <a
                                href="/profile/{post.user_id}"
                                class="btn btn-soft btn-block gap-2"
                            >
                                {#if post.profile_picture}
                                    <div class="avatar">
                                        <div class="w-8 h-8 rounded-full">
                                            <img
                                                src={post.profile_picture}
                                                alt={post.user_name}
                                            />
                                        </div>
                                    </div>
                                {:else}
                                    <div class="avatar placeholder">
                                        <div
                                            class="bg-neutral-focus text-neutral-content rounded-full w-4 h-4"
                                        >
                                            <span class="text-xs"
                                                >{post.user_name
                                                    .charAt(0)
                                                    .toUpperCase()}</span
                                            >
                                        </div>
                                    </div>
                                {/if}
                                <span>By: {post.user_name}</span>
                            </a>
                        </div>
                    {/if}
                </div>
            </div>
        {/each}
    </div>
    <!-- </div> -->
</div>
