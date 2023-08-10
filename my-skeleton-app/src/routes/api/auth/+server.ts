import { error, type RequestEvent } from '@sveltejs/kit';

export async function GET(requestEvent: RequestEvent) {
	const { request, url, fetch } = requestEvent;
	const code = url.searchParams.get('code');
	if (!code) {
		throw error(401, 'Code not found in URL');
	}

	// Need to do this by hand since we chain several requests
	let cookies = request.headers.get('cookie');
	if (cookies == null) {
		cookies = '';
	} else {
		const pos = cookies.search('user_id');
		if (pos < 0) {
			cookies = '';
		}
	}
	const res = await fetch(`http://127.0.0.1:8000/auth/token/${code}`, {
		method: 'GET',
		credentials: 'include',
		headers: { cookie: cookies },
	});
	return res;
}
