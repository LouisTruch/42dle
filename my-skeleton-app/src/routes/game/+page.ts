import type { PageLoadEvent } from './$types';

export const load = async (loadEvent: PageLoadEvent) => {
	const { fetch } = loadEvent;

	const res = await fetch('api/users');
	const everyUser = await res.json();

	const leaderboard = await fetch('api/leaderboard');
	const leaderboardUsers = await leaderboard.json();
	return { leaderboardUsers, everyUser };
};
