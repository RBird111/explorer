<script lang="ts">
	import type { OsFile } from '$lib';

	export let file: OsFile;

	const getIcon = (file_type: string) =>
		file_type === 'file' ? 'fa-regular fa-file' : 'fa-regular fa-folder';
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

		&,
		& > * {
			transition: all 0.4s ease-in-out;
		}

		i {
			font-size: 6vmin;
		}

		p {
			margin-top: 1vmin;
			font-size: 2.4vmin;
			text-align: center;
			text-overflow: ellipsis;
			white-space: nowrap;
			width: 90%;
			overflow: hidden;
		}
	}

	.file:hover {
		i,
		p {
			color: $c-acc;
		}

		i {
			transform: scale(1.3);
		}
	}
</style>
