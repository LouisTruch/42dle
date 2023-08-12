import { env } from '../env.js';

export const GET = async ({ fetch, request }) => {
	let cookies = request.headers.get('cookie');
	if (cookies == null) {
		cookies = '';
	} else {
		const pos = cookies.search('user_id');
		if (pos < 0) {
			cookies = '';
		}
	}

	const res = await fetch(env.api + '/game/student-users', {
		credentials: 'include',
		headers: { cookie: cookies },
	});

	return res;
};
