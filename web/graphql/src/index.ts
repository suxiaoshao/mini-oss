import { ApolloClient, InMemoryCache } from '@apollo/client';

export * from './types';
export const client = new ApolloClient({
  uri: 'http://api.mini-oss.top:30002',
  cache: new InMemoryCache(),
});
export { ApolloProvider } from '@apollo/client';
