import { authenticateUser } from '$lib/server/auth';
import type { Handle, HandleFetch } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	console.log('hooks.server:handle');
	event.locals.user = authenticateUser(event);

	const response = await resolve(event);

	return response;
};

export const handleFetch = (async ({ event, request, fetch }) => {
	console.log('hooks.server:handlefetch');

	return fetch(request);
}) satisfies HandleFetch;
