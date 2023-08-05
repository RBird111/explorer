<script lang="ts">
	import type { Directory } from '$lib';
	import File from '$lib/components/File.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let directory: Directory | null = null;

	onMount(async () => {
		directory = await invoke('get_curr_directory');
	});
</script>

<div class="wrap">
	{#if directory}
		<div class="directory">
			{directory.dir}
		</div>

		<div class="files">
			<File file={{ name: '..', file_type: 'directory' }} />
			{#each directory.files as file (file.name)}
				<File {file} />
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	.wrap {
		width: 100vw;
		height: 100vh;

		display: grid;
		grid-template-rows: 1fr 19fr;

		.directory {
			display: flex;
			justify-content: flex-start;
			align-items: center;

			font-weight: 400;

			background-color: $c-main;
			color: $c-background;
			padding: 0 0.3rem;
		}

		.files {
			display: grid;
			align-content: start;
			grid-template-columns: repeat(auto-fit, minmax(19.2vw, 1fr));
			// gap: 1vw;

			overflow-y: auto;
		}
	}
</style>
