<script lang="ts">
	import '$lib/styles/app.scss';
</script>

<div class="app-wrap">
	<slot />
</div>

<style lang="scss">
	.app-wrap {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}
</style>
