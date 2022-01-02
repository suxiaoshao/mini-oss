import { BrowserRouter } from 'react-router-dom';
import CustomTheme from '../../../common/src/CustomTheme';
import AppRouter from './components/AppRouter';
import { Provider } from 'react-redux';
import { store } from './app/store';
import { ApolloProvider, client } from 'graphql';

function App(): JSX.Element {
  return (
    <Provider store={store}>
      <ApolloProvider client={client}>
        <CustomTheme>
          <BrowserRouter>
            <AppRouter />
          </BrowserRouter>
        </CustomTheme>
      </ApolloProvider>
    </Provider>
  );
}

export default App;
