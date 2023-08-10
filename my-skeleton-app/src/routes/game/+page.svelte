<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData, PageData } from './$types';
	import { modalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings } from '@skeletonlabs/skeleton';

	export let data: PageData;
	const users = data.users;
	export let form: ActionData;

	export let modal: ModalSettings = {
		type: 'alert',
		title: '',
		body: '',
		buttonTextCancel: 'Close',
	};
</script>

<div class="w-80 block justify-center mx-auto items-center border-2 rounded-xl p-2">
	<img src="http://localhost:8000/game/guess-image" alt="a 42 student to guess" />
	<h1>GAMING</h1>
	<form method="POST" action="?/guess" use:enhance>
		<label class="label">
			<input class="input" name="login" type="text" value={form?.login ?? ''} />
		</label>
		{#if form?.missing}<p class="input-error">Missing field</p>{/if}
		<button>GUESS</button>
	</form>
</div>

<div class="w-60 block mx-auto table-container">
	<table class="table table-compact table-interactive">
		<thead>
			<tr>
				<th class="text-primary-500">Login</th>
				<th class="text-secondary-500">Points</th>
			</tr>
		</thead>
		<tbody class="">
			{#each users as user}
				<tr
					on:click={() => {
						modal.title = user.login;
						modal.image = user.profilePic;
						modalStore.trigger(modal);
					}}
				>
					<td class="text-primary-400">{user.login}</td>
					<td class="text-secondary-400">{user.score}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
