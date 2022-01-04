import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client';

export * from './types';

const link = createHttpLink({
  uri: import.meta.env.DEV ? '/graphql' : 'http://api.mini-oss.top:30002',
  credentials: 'include',
});
export const client = new ApolloClient({
  link,
  cache: new InMemoryCache(),
});
export { ApolloProvider } from '@apollo/client';
