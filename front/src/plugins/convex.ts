import { useAuth } from '@clerk/vue';
import { ConvexClient } from 'convex/browser';
import { ref, watch, type Plugin } from 'vue';

export const convex: Plugin = {
  install(app, { convexUrl }: { convexUrl: string }) {
    if (!convexUrl) {
      throw new Error('`convexUrl` is required to initialize Convex plugin');
    }

    try {
      const isAuthenticated = ref(false);
      const client = new ConvexClient(convexUrl);

      const { getToken, isSignedIn } = useAuth();

      function onSignedIn(value?: boolean) {
        if (!value) return;

        client.setAuth(
          async ({ forceRefreshToken }) => {
            try {
              return await getToken.value({
                template: 'convex',
                skipCache: forceRefreshToken,
              });
            } catch (err) {
              console.error('error getting auth token:', err);
              return null;
            }
          },
          (authenticated) => {
            isAuthenticated.value = authenticated;

            if (authenticated) {
              console.info('convex client authenticated');
            } else {
              console.warn('convex client not authenticated');
            }
          },
        );
      }

      watch(isSignedIn, (v) => onSignedIn(v), { immediate: true });

      app.provide('convexClient', client);
      app.provide('isConvexAuthenticated', isAuthenticated);
    } catch (error) {
      console.error('failed to create convex client:', error);
    }
  },
};
