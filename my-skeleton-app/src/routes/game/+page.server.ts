import type { PageServerLoad, Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	guess: async ({ request, fetch }) => {
		const data = await request.formData();
		const login = data.get('login');
		if (!login) {
			return fail(422, { login, missing: true });
		}
		const resUser = await fetch('api/users');
		const everyUser = await resUser.json();
		if (!everyUser.some((user: user) => user.login == login)) {
			return fail(422, { login, loginNotFound: true });
		}

		const body = JSON.stringify({ login_to_guess: login });
		const resApi = await fetch('/api/game', {
			method: 'POST',
			body: body,
			headers: {
				'x-sveltekit-action': 'true',
			},
		});
		if (!resApi.ok) {
			return fail(422, { login, wrong: true });
		}
		return { success: true };
	},
};

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(302, '/login');
	}
};

export interface user {
	login: string;
	first_name: string;
	last_name: string;
	profile_pic: string;
}
