import type { PageServerLoad } from './$types';

export const load = (async (event) => {
    const layoutCookie = event.cookies.get("PaneForge:layout");
	const collapsedCookie = event.cookies.get("PaneForge:collapsed");

	let layout: number[] | undefined;
	let collapsed: boolean | undefined;

	if (layoutCookie) layout = JSON.parse(layoutCookie);

	if (collapsedCookie) collapsed = JSON.parse(collapsedCookie);

	return { layout, collapsed };
}) satisfies PageServerLoad;