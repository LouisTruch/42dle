import { redirect } from '@sveltejs/kit';

export const load = async ({ fetch, url }) => {
	let codeAfterRedirect = url.searchParams.get('code');

	const res = await fetch(`/api/auth?code=${codeAfterRedirect}`);
	throw redirect(302, '/profile');
	// I dont understand throwing I guess
};
