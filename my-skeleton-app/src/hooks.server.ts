import type { Handle, HandleFetch } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const userInfo = await getUserInfo(event);
	if (userInfo.login == '') {
		return await resolve(event);
	}

	event.locals.user = {
		login: userInfo.login,
		profile_pic: userInfo.profile_pic,
	};

	return await resolve(event);
};

async function getUserInfo(event: any) {
	const session = event.cookies.get('user_id');
	const res = await event.fetch('http://localhost:8000/auth/info', {
		credentials: 'include',
		headers: { cookie: session },
	});
	if (res.ok) {
		const jsonInfo = await res.json();
		return {
			login: jsonInfo.login,
			profile_pic: jsonInfo.profile_pic,
		};
	}
	return {
		login: '',
		profile_pic: '',
	};
}
// export const handleFetch = (async ({ event, request, fetch }) => {
// 	console.log('hooks.server:handlefetch');
// 	return fetch(request);
// }) satisfies HandleFetch;
