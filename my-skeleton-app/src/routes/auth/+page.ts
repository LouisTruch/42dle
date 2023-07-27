import type { PageLoadEvent } from './$types';
import { redirect } from '@sveltejs/kit';

export const load = async (loadEvent: PageLoadEvent) => {
	const { url, fetch } = loadEvent;
	let codeAfterRedirect = url.searchParams.get('code');

	if (codeAfterRedirect == null || codeAfterRedirect.length == 0) {
		throw redirect(307, '/');
	}

	const response = await fetch(`http://localhost:8000/auth/token/${codeAfterRedirect}`);
	console.log(response);

	throw redirect(303, '/profile');
};
