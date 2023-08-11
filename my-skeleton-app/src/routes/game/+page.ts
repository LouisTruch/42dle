import type { PageLoadEvent } from './$types';

export const load = async (loadEvent: PageLoadEvent) => {
	const { fetch } = loadEvent;

	const res = await fetch('api/users');
	if (res.ok) {
		// change this when leaderboard is done
		var everyUser = await res.json();
	}

	// const lbRes = await fetch('api/leaderboard');
	// const json = await lbRes.json();
	// console.log(json);
	//Need to create leaderboard there

	//Change this to our rust API leaderboard
	const response = await fetch('http://localhost:4000/users');
	const users = await response.json();
	return { users, everyUser };
};
