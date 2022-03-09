import { createHttpLink } from '@apollo/client';
import { onError } from '@apollo/client/link/error';
import { enqueueSnackbar } from '..';

/**  */
export const link = createHttpLink({
  uri: String(import.meta.env.VITE_GRAPHQL_URL ?? 'http://api.mini-oss.sushao.top'),
  credentials: 'include',
});

/** 错误处理  */
export const errorLink = onError(({ graphQLErrors, networkError }) => {
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
