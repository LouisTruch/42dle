<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData, PageData } from './$types';
	import { Autocomplete, popup, toastStore } from '@skeletonlabs/skeleton';
	import type { AutocompleteOption, PopupSettings, ToastSettings } from '@skeletonlabs/skeleton';
	import type { user } from './+page.server';

	export let data: PageData;
	const leaderboardUsers = data.leaderboardUsers;
	const everyUser = data.everyUser;
	export let form: ActionData;

	let popupSetting: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom',
	};

	let toastSetting: ToastSettings = {
		message: '',
		// background: 'variant-filled-error',
		background: 'bg-gradient-to-tr from-indigo-500 via-purple-500 to-pink-500 text-white',
		// Add your custom classes here:
		classes: 'border-4 border-purple-500',
	};

	let input: string = '';
	$: inputSize = input.length;

	const loginOptions: AutocompleteOption[] = [];
	for (let user of everyUser) {
		const userInOptions = {
			label: user.login,
			value: user.login,
			keywords: user.first_name + ', ' + user.last_name,
		};
		loginOptions.push(userInOptions);
	}

	function onLoginSelection(event: any) {
		input = event.detail.label;
	}

	let imgSrc = 'http://localhost:8000/game/guess-image?';
	// Cache breaker to force the browser to make the request on imgSrc again
	let count = 0;
	$: cacheImgSrc = imgSrc + count;

	function handleClick() {
		setTimeout(() => {
			count++;

			if (!everyUser.some((user: user) => user.login == input)) {
				toastSetting.message = 'Not a valid login';
				toastStore.trigger(toastSetting);
			} else if (!form?.success) {
				toastSetting.message = 'WRONG';
				toastStore.trigger(toastSetting);
			}
		}, 200);
	}
</script>

<head><script src="https://kit.fontawesome.com/ad4e238733.js" crossorigin="anonymous"></script></head>
<div class="flex">
	<div class="bg-secondary-800 w-100 block justify-center mx-auto items-center p-2">
		<div class="h2 uppercase bg-secondary-900 font-bold"><p>Guess the student</p></div>
		<img src={cacheImgSrc} alt="a 42 student to guess" />
		<form method="POST" action="?/guess" use:enhance>
			<div class="flex">
				<input
					required
					autocomplete="off"
					class="select rounded-none rounded-bl-lg"
					name="login"
					type="text"
					placeholder="Login..."
					bind:value={input}
					use:popup={popupSetting}
				/>
				<div data-popup="popupAutocomplete" class="card max-w-sm overflow-y-auto w-full" tabindex="-1">
					{#if inputSize > 1}
						<Autocomplete bind:input options={loginOptions} on:selection={onLoginSelection} />
					{/if}
				</div>
				<button
					on:click={handleClick}
					class="btn bg-gradient-to-br variant-gradient-secondary-primary rounded-none rounded-br-lg"
					><i class="fa-solid fa-arrow-right" /></button
				>
			</div>
			{#if form?.loginNotFound}<p class="card">Enter a valid login :)</p>{/if}
			{#if form?.wrong}<p class="input-error">WRONG GROS FDP</p>{/if}
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
				{#each leaderboardUsers.splice(0, 4) as user}
					<tr>
						<td class="text-primary-400">{user.login}</td>
						<td class="text-secondary-400">{user.score}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
