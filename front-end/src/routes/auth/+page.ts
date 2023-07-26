import { redirect } from '@sveltejs/kit';

export const load = async (loadEvent) => {
	const { url } = loadEvent;
	let code = url.searchParams.get('code');

	if (code == null || code.length == 0) {
		throw redirect(307, '/');
	}

	return { code };
};
