import type { PageLoad } from './auth/$types';

export const load = (async ({ fetch }) => {
	const res = await fetch('/api/user?login=ltruchel');
	const json = await res.json();
	return {
		user: {
			login: json.login,
			profilePic: json.profilePic,
		},
	};
}) satisfies PageLoad;
