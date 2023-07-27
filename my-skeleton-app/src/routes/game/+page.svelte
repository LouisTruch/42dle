<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData, PageData } from './$types';

	export let data: PageData;
	export let form: ActionData;
</script>

<div class="w-40 block justify-center mx-auto items-center">
	<h1>GAMING</h1>
	<form method="POST" action="?/guess" use:enhance>
		<label class="label">
			<input class="input" name="login" type="text" value={form?.login ?? ''} />
		</label>
		{#if form?.missing}<p class="input-error">Missing field</p>{/if}
		<button disabled={form?.valid}>GUESS</button>
	</form>
</div>
