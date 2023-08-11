import type { PageServerLoad, Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	guess: async ({ request, fetch }) => {
		const data = await request.formData();
		const login = data.get('login');
		if (!login) {
			return fail(422, { login, missing: true });
		}
		const body = JSON.stringify({ login_to_guess: login });
		const res = await fetch('/api/game', {
			method: 'POST',
			body: body,
			headers: {
				'x-sveltekit-action': 'true',
			},
		});
	},
};

export const load: PageServerLoad = async ({ locals, fetch }) => {
	if (!locals.user) {
		throw redirect(302, '/login');
	}
};
