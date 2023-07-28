export async function externalFetch(request) {
	console.log(request.headers);
}

import { authenticateUser } from '$lib/server/auth';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	event.locals.user = authenticateUser(event);

	const response = await resolve(event);
	console.log(response);

	return response;
};