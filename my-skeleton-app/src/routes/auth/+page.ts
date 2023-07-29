import type { PageLoadEvent } from './$types';
import { redirect } from '@sveltejs/kit';

export const load = async (loadEvent: PageLoadEvent) => {
	const { url, fetch } = loadEvent;
	let codeAfterRedirect = url.searchParams.get('code');

	/*
	Louis, le document est undefine quand je fais ça.
	donc ça marche pas .

	let cookieList = document.cookie.split(';')
	let cookieName: string = "";
	for (let i = 0; i < cookieList.length; i++) {
    	const cookie = cookieList[i].trim();
    	if (cookie.startsWith(`user_id=`)) {
			cookieName = cookieList[i];
    	}
	}*/
	if (codeAfterRedirect == null || codeAfterRedirect.length == 0) {
		throw redirect(307, '/');
	}
	await fetch(`http://127.0.0.1:8000/auth/token/${codeAfterRedirect}`, {
		credentials: 'include', 
	})
	// .then(async (response) => {
	// 	// console.log(response.headers.getSetCookie());
	// 	const body: string[] = response.headers.getSetCookie();
	// 	await fetch('/api', { method: 'POST',body: String(body) }).then((response) => {
	// 		throw redirect(302, '/profile');
	// 	});
	// });
	// .catch((error) => {
	// 	throw redirect(302, '/');
	// });
};
