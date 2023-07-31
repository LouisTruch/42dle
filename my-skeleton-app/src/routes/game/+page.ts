import type { PageLoadEvent } from './$types';

export const load = async (loadEvent: PageLoadEvent) => {
	const { fetch } = loadEvent;

    //Change this to our rust API 
	const response = await fetch('http://localhost:4000/users');
	const users = await response.json();
	return { users };
};
