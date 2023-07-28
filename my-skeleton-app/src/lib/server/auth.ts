import type { RequestEvent } from '@sveltejs/kit';

// Make request to backend to check if session cookie is good, send back username if so or
// null if cookie is expired/bad
export const authenticateUser = (event: RequestEvent) => {
	return null;
};
