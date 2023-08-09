<script lang="ts">
	import { user } from '../store';
	import type { PageData } from './$types';

	export let data: PageData;
	user.set(data.user.login);
</script>

<div class="container h-full mx-auto flex justify-center items-top">
	<h1>Welcome to 42dle</h1>
</div>
