import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies, url }) => {
    const theme = cookies.get('theme') || 'lemonade';

    return {
        theme,
        url: { pathname: url.pathname },
        authenticated: false, // Will be handled by client-side checks
        user_id: null
    };
};
