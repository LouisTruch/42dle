import type { RequestEvent } from '@sveltejs/kit';
import { env } from '../env';

export async function POST(requestEvent: RequestEvent) {
	const { request, fetch } = requestEvent;

	let cookies = request.headers.get('cookie');
	if (cookies == null) {
		cookies = '';
	} else {
		const pos = cookies.search('user_id');
		if (pos < 0) {
			cookies = '';
		}
	}

	const formData = new FormData();
	formData.append('login_to_guess', 'me');
	const res = await fetch(env.api + '/game/', {
		method: 'POST',
		body: formData,
		credentials: 'include',
		headers: { cookie: cookies },
	});
	return res;
}
