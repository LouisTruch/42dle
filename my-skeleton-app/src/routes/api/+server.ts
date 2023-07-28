export async function POST(requestEvent) {
	const { request } = requestEvent;
	const body = await request.text();
	const header = new Headers();
	header.set('set-cookie', body);
	const response = new Response(null, { status: 200, headers: header });
	return response;
}
