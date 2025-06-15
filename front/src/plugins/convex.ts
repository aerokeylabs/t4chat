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

      app.runWithContext(() => {
        const { getToken, isSignedIn } = useAuth();

        function onAuthenticated(value?: boolean) {
          isAuthenticated.value = value ?? false;

          if (value) {
            console.info('convex client authenticated');
          } else {
            console.warn('convex client not authenticated');
          }
        }

        function onSignedIn(value?: boolean) {
          if (!value) {
            client.setAuth(() => Promise.resolve(null), onAuthenticated);
            return;
          }

          client.setAuth(async ({ forceRefreshToken }) => {
            try {
              return await getToken.value({
                template: 'convex',
                skipCache: forceRefreshToken,
              });
            } catch (err) {
              console.error('error getting auth token:', err);
              return null;
            }
          }, onAuthenticated);
        }

        watch(isSignedIn, (v) => onSignedIn(v), { immediate: true });
      });

      app.provide('convexClient', client);
      app.provide('isConvexAuthenticated', isAuthenticated);
    } catch (error) {
      console.error('failed to create convex client:', error);
    }
  },
};
