import { json, redirect } from '@sveltejs/kit';

export const load = async (loadEvent) => {
	const { url, fetch } = loadEvent;
	let code = url.searchParams.get('code');

	if (code == null || code.length == 0) {
		throw redirect(307, '/');
	}

	const response = await fetch(`http://localhost:8000/auth/token/${code}`);
	const token = await response.json();
	const access_token = await token.access_token;
	console.log(token);
	console.log(access_token);
	const resp = await fetch(`http://localhost:8000/auth/users/${access_token}`);
	const responseV2Me = await resp.json();
	console.log(responseV2Me);

	throw redirect(307, '/profile');
};
