import { ApolloClient, from, InMemoryCache } from '@apollo/client';
import { errorLink, link } from 'common/src/graphql';
import { setContext } from '@apollo/client/link/context';

const authLink = setContext((_, { headers }) => {
  // get the authentication token from local storage if it exists
  const token = window.localStorage.getItem('admin_auth');
  // return the headers to the context so httpLink can read them
  return {
    headers: {
      ...headers,
      authorization: token,
    },
  };
});

export const client = new ApolloClient({
  link: from([errorLink, authLink, link]),
  cache: new InMemoryCache(),
});
export { ApolloProvider } from '@apollo/client';
