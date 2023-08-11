<script lang="ts">
	import { preview, type OsFile } from '$lib';

	import { invoke } from '@tauri-apps/api/tauri';
	import File from '$lib/components/icons/File.svelte';
	import Folder from '$lib/components/icons/Folder.svelte';

	export let file: OsFile;
	let p: HTMLParagraphElement;
	$: hover = false;

	const onMouseEnter = () => {
		hover = true;
		p.style.color = file.file_type === 'file' ? '#c44056' : '#53c97a';
	};

	const onMouseLeave = () => {
		hover = false;
		p.style.color = '#f0ebd8';
	};

	const readFile = async () => {
		if (file.name !== '..') {
			preview.set(await invoke('read_file', { file: file.name }));
		}
	};
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="file"
	style:cursor={file.file_type === 'file' ? 'default' : 'pointer'}
	title={file.name}
	on:mouseenter={onMouseEnter}
	on:mouseleave={onMouseLeave}
	on:click={readFile}
>
	{#if file.file_type === 'file'}
		<File {hover} />
	{:else}
		<Folder {hover} />
	{/if}

	<p bind:this={p} class="file-name">{file.name}</p>
</div>

<style lang="scss">
	.file {
		@include txt-code;
		@include std-transition;

		height: 19.2vw;
		width: 100%;

		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;

		overflow: hidden;

		p {
			@include std-transition;

			margin-top: 1vmin;
			font-size: 2.4vmin;
			text-align: center;
			text-overflow: ellipsis;
			white-space: nowrap;
			width: 90%;
			overflow: hidden;
		}
	}
</style>
