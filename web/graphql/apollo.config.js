module.exports = {
  client: {
    service: {
      url: 'http://api.mini-oss.top:30002',
      name: 'graphql',
    },
    excludes: ['./src/types.ts'],
    includes: ['./src/schema/**/*.gql'],
  },
};
