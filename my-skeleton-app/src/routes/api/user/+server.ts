import type { RequestHandler } from '@sveltejs/kit';

export const GET = (async ({ url }) => {
	const user = url.searchParams.get('login');
	return new Response(`{
    "login": "ltruchel",
    "score": "100",
    "profilePic": "https://cdn.intra.42.fr/users/1e5eaf4ddd8d0024eed317458a390aff/ltruchel.jpg"
}`);
}) satisfies RequestHandler;
