<script lang="ts">
	import { type Directory, typewriter } from '$lib';

	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import File from '$lib/components/File.svelte';

	let directory: Directory | null = null;

	onMount(async () => (directory = await invoke('get_curr_directory')));

	const goUp = async () => (directory = await invoke('go_up'));

	const goDown = async (e: Event) => {
		let file = (e.currentTarget as HTMLButtonElement).value;
		directory = await invoke('go_down', { file });
	};
</script>

<div class="wrap">
	{#if directory}
		{#key directory.dir}
			<div class="directory" in:typewriter={{ speed: 6 }}>
				{directory.dir}
			</div>
		{/key}

		<div class="files">
			<button class="file-icon" value="What" on:click={goUp}>
				<File file={{ name: '..', file_type: 'directory' }} />
			</button>

			{#each directory.files as file (file.name)}
				<button class="file-icon" value={file.name} on:click={goDown}>
					<File {file} />
				</button>
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
			@include txt-code;

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
			grid-template-columns: repeat(auto-fit, 19.2vw);

			overflow-y: scroll;
		}
	}
</style>
