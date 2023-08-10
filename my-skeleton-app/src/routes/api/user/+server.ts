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

	const res = await fetch('http://localhost:8000/auth/info', {
		credentials: 'include',
		headers: { cookie: cookies },
	});

	return res;
};
