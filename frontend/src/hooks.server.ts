import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
    const mainDomain = 'skillswap.taneshq.iitmandi.in.net';

    const redirectDomains = [
        'www.skillswap.taneshq.me',
        'skillswap.taneshq.me',
        'www.skillswap.taneshq.iitmandi.in.net'
    ];

    const host = event.request.headers.get('host');

    if (host && redirectDomains.includes(host)) {
        const newUrl = `https://${mainDomain}${event.url.pathname}${event.url.search}`;

        return new Response(null, {
            status: 301,
            headers: {
                'Location': newUrl
            }
        });
    }

    const theme = event.cookies.get('theme') || 'lemonade';

    const response = await resolve(event, {
        transformPageChunk: ({ html }) => {
            const themeScript = `
                <script>
                    (function() {
                        const isPWA = window.matchMedia('(display-mode: standalone)').matches || 
                                      navigator.standalone;
                        
                        if (isPWA && !document.documentElement.getAttribute('data-theme')) {
                            const savedTheme = localStorage.getItem('theme');
                            if (savedTheme) {
                                document.documentElement.setAttribute('data-theme', savedTheme);
                            }
                        }
                    })();
                </script>
            `;
            
            return html
                .replace('data-theme=""', `data-theme="${theme}"`)
                .replace('</head>', `${themeScript}</head>`);
        }
    });

    return response;
};