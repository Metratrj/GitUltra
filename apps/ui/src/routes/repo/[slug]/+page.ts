import type { PageLoad } from './$types';

export const load = (async ({params}) => {

    return {
        title: params.slug,
    };
}) satisfies PageLoad;