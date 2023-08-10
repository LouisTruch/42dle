<script lang="ts">
	async function handleClick() {
		const res = await fetch('http://localhost:5173/api/admin/fill-db');
		console.log(res);
	}
	// Need to add alert to let admin know if function worked
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<button class="btn btn-bg variant-filled-secondary" on:click|once={handleClick}>FILL DB</button>
	<!-- <a href="http://localhost:8000/target" class="btn btn-bg variant-filled-primary" data-sveltekit-preload-data="off"
		><span><strong class="text-bg">generate d b</strong></span></a
	> -->
</div>
