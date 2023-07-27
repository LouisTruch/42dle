import { json, redirect } from '@sveltejs/kit';

export const load = async (loadEvent) => {
	const { url, fetch } = loadEvent;
	let code = url.searchParams.get('code');

	if (code == null || code.length == 0) {
		throw redirect(307, '/');
	}

	const response = await fetch(`http://localhost:8000/auth/token/${code}`).then(
	);
	console.log(response);

	throw redirect(302, '/login');
};
