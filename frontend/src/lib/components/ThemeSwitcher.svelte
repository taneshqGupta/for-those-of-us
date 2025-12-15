<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<script lang="ts">
	import { enhance } from '$app/forms';
	import { DropdownSvg, ThemeSvg } from './icons';
	export let currentPath: string;

	const themes = ['retro', 'valentine', 'forest', 'lemonade', 'dim'];

	let dropdownOpen = false;

	const submitUpdateTheme = () => {
		// @ts-ignore
		return async ({ action, update }) => {
			const theme = action.searchParams.get('theme');
			if (theme) {
				document.documentElement.setAttribute('data-theme', theme);
			}
			await update();
		};
	};
</script>

<div
	class="dropdown dropdown-end"
	on:focusin={() => (dropdownOpen = true)}
	on:focusout={() => (dropdownOpen = false)}
>
	<div
		tabindex="0"
		role="button"
		class={`btn btn-soft btn-sm w-14 ${dropdownOpen ? 'bg-base-200' : ''}`}
		aria-label="themes"
	>
		<ThemeSvg />
		<DropdownSvg />
	</div>
	<ul
		tabindex="0"
		class="p-2 shadow menu dropdown-content bg-base-100 rounded-box w-25 max-h-96 overflow-y-auto top-full mt-2"
		style="z-index: 9999;"
	>
		<form method="POST" use:enhance={submitUpdateTheme}>
			<li>
				<button
					class="btn btn-sm btn-ghost w-full justify-start normal-case"
					formaction="/?/setTheme&theme=lemonade&redirectTo={currentPath}"
					aria-label="lemonade"
				>
					Light
				</button>
			</li>
			<li>
				<button
					class="btn btn-sm btn-ghost w-full justify-start normal-case"
					formaction="/?/setTheme&theme=forest&redirectTo={currentPath}"
					aria-label="forest"
				>
					Dark
				</button>
			</li>
			<li>
				<button
					class="btn btn-sm btn-ghost w-full justify-start normal-case"
					formaction="/?/setTheme&theme=dim&redirectTo={currentPath}"
					aria-label="dim"
				>
					Dim
				</button>
			</li>
			<li>
				<button
					class="btn btn-sm btn-ghost w-full justify-start normal-case"
					formaction="/?/setTheme&theme=retro&redirectTo={currentPath}"
					aria-label="retro"
				>
					Retro
				</button>
			</li>
			<li>
				<button
					class="btn btn-sm btn-ghost w-full justify-start normal-case"
					formaction="/?/setTheme&theme=valentine&redirectTo={currentPath}"
					aria-label="valentine"
				>
					Valentine
				</button>
			</li>
		</form>
	</ul>
</div>
