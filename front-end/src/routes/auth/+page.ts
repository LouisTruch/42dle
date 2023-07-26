import { json, redirect } from '@sveltejs/kit';

export const load = async (loadEvent) => {
	const { url, fetch } = loadEvent;
	let code = url.searchParams.get('code');

	if (code == null || code.length == 0) {
		throw redirect(307, '/');
	}

	const response = await fetch(`http://localhost:8000/auth/token/${code}`);
	const token = await response.json();
	const test = token.access_token;
	console.log(token);
	console.log(test);

	return { code };
};
