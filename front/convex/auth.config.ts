export default {
  providers: [
    {
      // domain: process.env.VITE_CLERK_FRONTEND_API_URL,
      // for some reason the env doesnt load
      domain: "https://casual-satyr-90.clerk.accounts.dev/",
      applicationID: 'convex',
    },
  ],
};
