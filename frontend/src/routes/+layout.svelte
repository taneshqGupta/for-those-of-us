<script lang="ts">
    import "../app.css";
    import ThemeSwitcher from "$lib/components/ThemeSwitcher.svelte";
    import InstallAppButton from "$lib/components/InstallAppButton.svelte";
    import ProfilePicture from "$lib/components/ProfilePicture.svelte";
    import { GithubSvg } from "$lib/components/icons";
    import { authStore, initAuth, logout } from "$lib/auth";
    import { getMyProfile } from "$lib/api";
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import type { UserProfile } from "$lib/types";

    export let data;

    let userProfile: UserProfile | null = null;

    onMount(async () => {
        initAuth();

        if ($authStore.isAuthenticated) {
            try {
                userProfile = await getMyProfile();
            } catch (err) {
                console.log("Could not load user profile for header");
            }
        }
    });

    $: if ($authStore.isAuthenticated && !userProfile) {
        getMyProfile()
            .then((profile) => {
                userProfile = profile;
            })
            .catch(() => {
                console.log("Could not load user profile for header");
            });
    }

    async function handleLogout() {
        await logout();
    }

    $: isAuthPage = $page.url.pathname === "/login";
</script>

<div class="h-screen flex flex-col overflow-hidden bg-base-100">
    <header
        class="flex-none navbar bg-base-100 border-b border-base-300 relative"
        style="z-index: 1000;"
    >
        <h1 class="fancy-heading-font">
            <a
                href="/"
                class="btn btn-ghost font-bold text-2xl whitespace-nowrap"
                aria-label="Go to SkillSwap homepage"
            >
                <span class="not-lg:hidden"
                    >Skill-Swap: Learn, Teach, Socialise</span
                >
                <span class="lg:hidden">Skill-Swap</span>
            </a>
        </h1>
        <a href="/offer" class="not-md:hidden btn btn-soft btn-sm m-2"
            >Offer-Skill</a
        >
        <a href="/offer" class="md:hidden flex btn btn-soft btn-sm m-0.5">Offer</a
        >
        <a href="/request" class="not-md:hidden btn btn-soft btn-sm m-2"
            >Request-Help</a
        >
        <a href="/request" class="md:hidden btn btn-soft btn-sm ml-0.5 mr-1">Request</a>
        <div class="w-full"></div>
        <div class="flex items-center gap-2">
            <InstallAppButton />

            <nav aria-label="Theme Selection">
                <ThemeSwitcher
                    currentPath={data?.url?.pathname || $page.url.pathname}
                />
            </nav>

            {#if !isAuthPage && $authStore.isAuthenticated}
                {#if $authStore.user_id && userProfile}
                    <a
                        href="/profile/{$authStore.user_id}"
                        class="btn btn-circle btn-sm p-1"
                        aria-label="Go to profile"
                    >
                        <ProfilePicture
                            profilePicture={userProfile.profile_picture}
                            name={userProfile.name || "User"}
                            size="sm"
                            editable={false}
                        />
                    </a>
                {:else}
                    <div class="btn btn-circle btn-sm p-1 skeleton"></div>
                {/if}
            {/if}
        </div>
    </header>

    <main class="flex-1 overflow-hidden" id="main-content">
        <slot />
    </main>

    <footer
        class="footer sm:footer-horizontal bg-neutral text-neutral-content items-center p-4"
    >
        <!-- <aside class="grid-flow-col items-center">
            <SquirrelSvg />
            <a href="/about" class="link link-hover">About This Template</a>
        </aside> -->
        <nav
            class="grid-flow-col gap-4 md:place-self-center md:justify-self-end"
        >
            <a
                href="https://github.com/taneshqGupta/rust-svelte-template"
                class="link link-hover flex items-center gap-1"
                target="_blank"
                rel="noopener noreferrer"
                aria-label="View project on GitHub"
            >
                <GithubSvg />
                View on GitHub
            </a>
        </nav>
    </footer>
</div>
