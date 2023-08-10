import type { RequestEvent } from '@sveltejs/kit';

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
	const response = await fetch('http://127.0.0.1:8000/game/update-db', {
		method: 'GET',
		credentials: 'include',
		headers: { cookie: cookies },
	});
	return response;
}
