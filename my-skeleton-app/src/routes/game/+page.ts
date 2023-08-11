export const load = async ({ fetch }) => {
	const res = await fetch('api/users');
	const everyUser = await res.json();

	const leaderboard = await fetch('api/leaderboard');
	const leaderboardUsers = await leaderboard.json();
	return { leaderboardUsers, everyUser };
};
