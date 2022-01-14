import { SnackbarProvider, ApolloProvider, client, CustomTheme } from 'common';
import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import { BrowserRouter } from 'react-router-dom';
import App from './App';
import { store } from './app/store';

ReactDOM.render(
  <React.StrictMode>
    <Provider store={store}>
      <CustomTheme>
        <SnackbarProvider>
          <ApolloProvider client={client}>
            <BrowserRouter>
              <App />
            </BrowserRouter>
          </ApolloProvider>
        </SnackbarProvider>
      </CustomTheme>
    </Provider>
  </React.StrictMode>,
  document.getElementById('root'),
);
