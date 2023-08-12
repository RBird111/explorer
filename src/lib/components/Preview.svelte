<script lang="ts">
	import { preview } from '$lib';

	$: visible = false;
	let content: string | null = null;

	preview.subscribe((val) => {
		content = val;
		visible = content ? true : false;
	});

	const onClick = () => {
		preview.set(null);
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="wrap" class:visible on:click={onClick}>
	{content}
</div>

<style lang="scss">
	.wrap {
		@include txt-code;

        visibility: hidden;

		position: absolute;
		bottom: 0;
		left: 0;

		height: calc(100vh - 30px);
		width: 100vw;

        background-color: $c-background;

        overflow: auto;

		&.visible {
            visibility: visible;
		}
	}
</style>
