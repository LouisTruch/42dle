import type { Handle, HandleFetch } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const userInfo = await getUserInfo(event);
	if (userInfo.login == '') {
		return await resolve(event);
	}

	const userAdmin: boolean = await getAdminRights(event);

	event.locals.user = {
		login: userInfo.login,
		profile_pic: userInfo.profile_pic,
		isAdmin: userAdmin,
	};

	return await resolve(event);
};

async function getUserInfo(event: any) {
	const user_id = event.cookies.get('user_id');
	const res = await event.fetch('http://localhost:8000/auth/info', {
		credentials: 'include',
		headers: { cookie: user_id },
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

async function getAdminRights(event: any) {
	const user_id = event.cookies.get('user_id');
	const res = await event.fetch('http://localhost:8000/auth/admin', {
		credentials: 'include',
		headers: { cookie: user_id },
	});
	console.log(res);
	if (res.ok) {
		return true;
	}
	return false;
}
