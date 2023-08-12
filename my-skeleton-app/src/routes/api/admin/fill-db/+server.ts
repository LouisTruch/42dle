import type { RequestEvent } from '@sveltejs/kit';
import { env } from '../../env';

export async function GET(requestEvent: RequestEvent) {
	const { fetch, request } = requestEvent;

	let cookies = request.headers.get('cookie');
	if (cookies == null) {
		cookies = '';
	} else {
		const pos = cookies.search('user_id');
		if (pos < 0) {
			cookies = '';
		}
	}
	const response = await fetch(env.api + '/game/update-pool-db', {
		method: 'GET',
		credentials: 'include',
		headers: { cookie: cookies },
	});
	return response;
}
