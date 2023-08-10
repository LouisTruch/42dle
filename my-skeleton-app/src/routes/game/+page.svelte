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

	let imgSrc = 'http://localhost:8000/game/guess-image?';
	$: cacheImgSrc = imgSrc;

	function handleClick() {
		// Cache breaker to force the browser to make the request on imgSrc again
		$: cacheImgSrc = imgSrc + new Date().getTime();
	}
</script>

<div class="w-80 block justify-center mx-auto items-center border-2 rounded-xl p-2">
	<img src={cacheImgSrc} alt="a 42 student to guess" />
	<form method="POST" action="?/guess" use:enhance>
		<label class="label">
			<input class="input" name="login" type="text" value={form?.login ?? ''} />
		</label>
		{#if form?.missing}<p class="input-error">Missing field</p>{/if}
		<button on:click={handleClick} class="btn variant-filled">GUESS STUDENT</button>
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
