<script lang="ts">
	import { type Directory, typewriter } from '$lib';

	import { appWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import File from '$lib/components/File.svelte';
	import MinIcon from '$lib/assets/MinIcon.svelte';
	import MaxIcon from '$lib/assets/MaxIcon.svelte';
	import CloseIcon from '$lib/assets/CloseIcon.svelte';

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
		<div data-tauri-drag-region class="title-bar">
			{#key directory.dir}
				<div class="directory" in:typewriter={{ speed: 6 }}>
					{directory.dir}
				</div>
			{/key}

			<div class="title-buttons">
				<button
					class="title-button"
					on:click={() => {
						appWindow.minimize();
					}}
				>
					<MinIcon />
				</button>

				<button
					class="title-button"
					on:click={() => {
						appWindow.maximize();
					}}
				>
					<MaxIcon />
				</button>

				<button
					class="title-button"
					on:click={() => {
						appWindow.close();
					}}
				>
					<CloseIcon />
				</button>
			</div>
		</div>

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

		.title-bar {
			@include txt-code;

			display: flex;
			justify-content: space-between;
			align-items: center;
			font-weight: 400;
			background-color: $c-main;
			color: $c-background;
			padding: 0 5px;
			height: 30px;

			.title-buttons {
				display: flex;

				.title-button {
					display: flex;
					justify-content: center;
					align-items: center;
				}
			}
		}

		.files {
			display: grid;
			align-content: start;
			grid-template-columns: repeat(auto-fit, 19.2vw);

			overflow-y: scroll;
		}
	}
</style>
