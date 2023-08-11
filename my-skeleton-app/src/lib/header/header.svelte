<script lang="ts">
	import { AppBar, Avatar } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import { goto, invalidateAll } from '$app/navigation';

	async function handleLogout() {
		const res = await fetch('/api/logout');
		// Goto not working
		goto('/login');
		invalidateAll();
	}
</script>

<AppBar padding="p-2" class="w-full">
	<svelte:fragment slot="lead"
		><strong class="text-xl uppercase hover:!text-primary-500"><a href="/">42dle</a></strong>
		{#if $page.data.user}
			<div class="px-2">
				<Avatar
					border="border-2 border-surface-300-500-token"
					initials="42"
					width="w-12"
					rounded="rounded-full"
					src={$page.data.user.profile_pic}
				/>
			</div>
			<h2 class="hover:!text-primary-600"><a href="/game">Game</a></h2>
			<button on:click={handleLogout}><h2 class=" px-4 hover:!text-primary-500">Logout</h2></button>
			{#if $page.data.user.isAdmin}
				<h2 class="hover:!text-primary-500"><a href="/admin">Admin</a></h2>
			{/if}
		{/if}

		{#if !$page.data.user}
			<h2><a class="px-4 hover:!text-primary-500" href="/login">Login</a></h2>
		{/if}
	</svelte:fragment>

	<svelte:fragment slot="default" />

	<svelte:fragment slot="trail">
		<strong class="text-l p-2 hover:!text-primary-500"><a href="/about">About</a></strong></svelte:fragment
	>
</AppBar>
