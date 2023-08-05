<script lang="ts">
	import type { OsFile } from '$lib';

	export let file: OsFile;

	const getIcon = (file_type: string) =>
		file_type === 'file'
			? 'fa-regular fa-file'
			: file_type === 'directory'
			? 'fa-regular fa-folder'
			: 'fa-solid fa-link';
</script>

<div class="file">
	<i class={getIcon(file.file_type)} />
	<p class="file-name">{file.name}</p>
</div>

<style lang="scss">
	.file {
		@include txt-code;

		height: 19.2vw;
		width: 100%;

		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;

		border: 1px solid $c-gray;
		overflow: hidden;

		i {
			font-size: 6vmin;
		}

		p {
			margin-top: 1vmin;
			font-size: 2.3vmin;
			text-align: center;
			text-overflow: ellipsis;
			white-space: nowrap;
			width: 90%;
			overflow: hidden;
		}
	}
</style>
