import { createHttpLink, ApolloClient, InMemoryCache, from } from '@apollo/client';
import { onError } from '@apollo/client/link/error';
import { enqueueSnackbar } from '..';

/**  */
const link = createHttpLink({
  uri: 'http://api.mini-oss.top',
  credentials: 'include',
});

/** 错误处理  */
const errorLink = onError(({ graphQLErrors, networkError }) => {
  if (graphQLErrors) {
    graphQLErrors.forEach(({ message, locations, path, extensions }) => {
      enqueueSnackbar(message);
      console.log(`[GraphQL error]: Message: ${message}, Location: ${locations}, Path: ${path}
source: ${extensions['source']}`);
    });
  }
  if (networkError) {
    enqueueSnackbar(`网络错误:${networkError.message}`);
    console.log(`[Network error]: ${networkError}`);
  }
});
export const client = new ApolloClient({
  link: from([errorLink, link]),
  cache: new InMemoryCache(),
});
export { ApolloProvider } from '@apollo/client';
