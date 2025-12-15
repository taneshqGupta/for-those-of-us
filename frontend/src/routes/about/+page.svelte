<script>
	import { RustSvg, SvelteKitSvg, PostgresSvg, GithubSvg } from '$lib/components/icons';
</script>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold text-center mb-8">About This Template</h1>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
		<div
			class="card bg-base-200 shadow-xl p-2 transition-all duration-500 ease-in-out transform"
			data-aos="fade-up"
		>
			<div class="card-body">
				<h2 class="card-title text-2xl font-bold mb-4">Overview</h2>
				<p>This is a starter CRUD template for Rust and SvelteKit.</p>
				<p class="mt-2"><strong>Creation:</strong> 2025</p>
				<p><strong>By:</strong> Taneshq Gupta (CSE Undergrad @ IIT-Mandi)</p>
			</div>
		</div>

		<div
			class="card bg-base-200 shadow-xl p-2 transition-all duration-500 ease-in-out transform"
			data-aos="fade-up"
			data-aos-delay="150"
		>
			<div class="card-body">
				<h2 class="card-title text-2xl font-bold mb-4">Key Features</h2>
				<ul class="list-disc list-inside ml-4 space-y-2">
					<li>Easily installable as a <strong>PWA (Progressive Web-Application)</strong>.</li>
					<li><strong>Dynamic + Persistent Theme-Switching</strong> with a multi-theme setup.</li>
				</ul>
			</div>
		</div>

		<div
			class="card bg-base-200 shadow-xl p-2 md:col-span-2 transition-all duration-500 ease-in-out transform"
			data-aos="fade-up"
			data-aos-delay="300"
		>
			<div class="card-body">
				<h2 class="card-title text-2xl font-bold mb-4">Tech Stack</h2>
				<div class="grid grid-cols-1 sm:grid-cols-1 gap-4 md:grid-cols-3">
					<div class="stat">
						<div class="stat-figure"><RustSvg /></div>
						<div class="stat-title">Backend</div>
						<div class="stat-value text-2xl">Rust</div>
					</div>
					<div class="stat">
						<div class="stat-figure text-secondary"><SvelteKitSvg /></div>
						<div class="stat-title">Frontend</div>
						<div class="stat-value text-2xl">SvelteKit</div>
					</div>
					<div class="stat">
						<div class="stat-figure text-secondary"><PostgresSvg /></div>
						<div class="stat-title">Database</div>
						<div class="stat-value text-2xl">Postgres</div>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="text-center mt-12" data-aos="fade-up" data-aos-delay="450">
		<a
			href="https://github.com/taneshqGupta/rust-svelte-template"
			class="btn btn-soft btn-lg gap-2"
			target="_blank"
			rel="noopener noreferrer"
			aria-label="View Template on GitHub"
		>
			<GithubSvg />
			<h3>View Template on GitHub</h3>
		</a>
	</div>
</div>
