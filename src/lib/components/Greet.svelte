<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let greetMsg = '';

	async function greet() {
		greetMsg = await invoke('greet', { name });
	}
</script>

<div class="wrap">
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<button on:click={greet}>Greet</button>
	<p>{greetMsg}</p>
</div>

<style lang="scss">
	.wrap {
		display: flex;
		flex-flow: column wrap;
		align-items: center;
		justify-content: center;

		& > * {
			margin-top: 0.5rem;
			padding: 0.3rem;
			width: 100%;
		}

		p {
			text-align: center;
		}
	}
</style>
