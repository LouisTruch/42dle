import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
	const isAdmin = locals.user.isAdmin;
	if (!locals.user || !isAdmin) {
		throw redirect(302, '/login');
	}
};
