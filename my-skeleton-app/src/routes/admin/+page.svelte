<script lang="ts">
	async function fillDbStud() {
		const res = await fetch('http://localhost:5173/api/admin/fill-db');
	}
	async function fillDbPool() {}
	// Need to add alert to let admin know if function worked
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<button class="btn btn-bg variant-filled-secondary" on:click|once={fillDbStud}>FILL DB STUD</button>
	<button class="btn btn-bg variant-filled-secondary" on:click|once={fillDbPool}>FILL DB POOL</button>

	<!-- <a href="http://localhost:8000/target" class="btn btn-bg variant-filled-primary" data-sveltekit-preload-data="off"
		><span><strong class="text-bg">generate d b</strong></span></a
	> -->
</div>
