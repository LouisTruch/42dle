import { error, type RequestEvent } from '@sveltejs/kit';

export async function POST(requestEvent: RequestEvent) {
	const { request } = requestEvent;
	const body = await request.text();
	const header = new Headers();
	header.set('set-cookie', body);
	const response = new Response(null, { status: 200, headers: header });
	return response;
}

export async function GET(requestEvent: RequestEvent) {
	const { request, url, fetch } = requestEvent;
	const code = url.searchParams.get('code');
	if (!code) {
		throw error(401, 'Code not found in URL');
	}
	
	let cookies = request.headers.get('cookie');
	if (cookies == null) {
		cookies = '';
	} else {
		const pos = cookies.search('user_id');
		if (pos < 0) {
			cookies = '';
		}
	}

	const response = await fetch(`http://127.0.0.1:8000/auth/token/${code}`, {
		method: 'GET',
		credentials: 'include',
		headers: { cookie: cookies },
	});
	return response;
}
