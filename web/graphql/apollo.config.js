module.exports = {
  client: {
    service: {
      url: 'http://api.mini-oss.top:80',
      name: 'graphql',
    },
    excludes: ['./src/types.ts'],
    includes: ['./src/schema/**/*.gql'],
  },
};
