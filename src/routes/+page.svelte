<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	type Dir = {
		directory: string;
		files: {
			name: string;
			file_type: 'directory' | 'file' | 'symlink';
		}[];
	};

	let dir: Dir | null = null;

	onMount(async () => {
		dir = await invoke('get_curr_directory');
	});
</script>

<div class="wrap">
	Hello

	{#if dir}
		<div class="directory">
			{dir.directory}
		</div>

		<div class="files">
			{#each dir.files as file}
				<p>{file.name}</p>
				<p>{file.file_type}</p>
			{/each}
		</div>
	{/if}
</div>

<style lang="scss">
	.wrap {
		width: 100vw;
		height: 100vh;
	}
</style>
