<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData, PageData } from './$types';
	import { modalStore, Autocomplete, popup } from '@skeletonlabs/skeleton';
	import type { ModalSettings, AutocompleteOption, PopupSettings } from '@skeletonlabs/skeleton';

	export let data: PageData;
	const leaderboardUsers = data.leaderboardUsers;
	const everyUser = data.everyUser;
	export let form: ActionData;

	let popupSetting: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom',
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

<div class="flex">
	<div class="bg-secondary-800 w-100 block justify-center mx-auto items-center p-2">
		<div class="h2 uppercase bg-secondary-900 font-bold"><p>Guess the student :</p></div>
		<img src={cacheImgSrc} alt="a 42 student to guess" />
		<form method="POST" action="?/guess" use:enhance>
			<div class="flex">
				<input
					required
					autocomplete="off"
					class="select"
					name="login"
					type="text"
					placeholder="stud..."
					bind:value={input}
					use:popup={popupSetting}
				/>
				<div data-popup="popupAutocomplete" class="card max-w-sm overflow-y-auto w-full" tabindex="-1">
					{#if inputSize > 1}
						<Autocomplete bind:input options={loginOptions} on:selection={onLoginSelection} />
					{/if}
				</div>
				<!-- {#if form?.missing}<p class="input-error">Missing field</p>{/if} -->
				<button on:click={handleClick} class="btn bg-gradient-to-br variant-gradient-secondary-primary rounded-xl"
					>?</button
				>
			</div>
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
