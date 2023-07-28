<script lang="ts">
	import { redirect } from '@sveltejs/kit';

	const test = async () => {
		console.log('test');
		await fetch('http://localhost:8000/aaa', {
			credentials: 'include',
		})
			.then((response) => {
				console.log(response.headers);
			})
			.catch((error) => {
				throw redirect(300, '/profile');
			});
	};
</script>

<div class="container h-full mx-auto flex justify-center items-top">
	<h1>Welcome to 42dle</h1>
</div>
