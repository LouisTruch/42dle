import type { RequestEvent } from '@sveltejs/kit';

export async function POST(requestEvent: RequestEvent) {
	const { request, fetch } = requestEvent;

	console.log(request.text);
	let cookies = request.headers.get('cookie');
	if (cookies == null) {
		cookies = '';
	} else {
		const pos = cookies.search('user_id');
		if (pos < 0) {
			cookies = '';
		}
	}
	// Check if back receive cookie there
	const formData = new FormData();
	formData.append('login_to_guess', 'me');
	const res = await fetch(`http://127.0.0.1:8000/game/`, {
		method: 'POST',
		body: formData,
		credentials: 'include',
		headers: { cookie: cookies },
	});
	return res;
}
