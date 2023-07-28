import type { PageServerLoad, Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	guess: async ({ request }) => {
		console.log('test');

		const data = await request.formData();
		const login = data.get('login');
		if (!login) {
			return fail(400, { login, missing: true });
		}
	},
};

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(303, '/login');
	}
};
