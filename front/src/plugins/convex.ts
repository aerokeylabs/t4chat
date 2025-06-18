import { useAuth } from '@clerk/vue';
import { ConvexClient } from 'convex/browser';
import { ref, watch, type Plugin } from 'vue';

export const CONVEX_CLIENT_KEY = 'convexClient';
export const IS_CONVEX_SIGNED_IN_KEY = 'isConvexSignedIn';
export const IS_CONVEX_LOADING_KEY = 'isConvexLoading';

export const convex: Plugin = {
  install(app, { convexUrl }: { convexUrl: string }) {
    if (!convexUrl) {
      throw new Error('`convexUrl` is required to initialize Convex plugin');
    }

    app.runWithContext(() => {
      try {
        const isConvexSignedIn = ref(false);
        const isConvexLoading = ref(true);
        const client = new ConvexClient(convexUrl);

        const { getToken, isSignedIn } = useAuth();

        function onChange(value?: boolean) {
          isConvexSignedIn.value = value ?? false;
          isConvexLoading.value = value == null;

          if (value) {
            console.info('convex client authenticated');
          } else {
            console.warn('convex client not authenticated');
          }
        }

        function onSignedIn(value?: boolean) {
          if (!value) {
            client.setAuth(() => Promise.resolve(null), onChange);
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
          }, onChange);
        }

        watch(isSignedIn, (v) => onSignedIn(v), { immediate: true });

        app.provide(CONVEX_CLIENT_KEY, client);
        app.provide(IS_CONVEX_SIGNED_IN_KEY, isConvexSignedIn);
        app.provide(IS_CONVEX_LOADING_KEY, isConvexLoading);
      } catch (error) {
        console.error('failed to create convex client:', error);
      }
    });
  },
};
