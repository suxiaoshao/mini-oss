import { ApolloProvider } from '@apollo/client';
import { SnackbarProvider, CustomTheme } from 'common';
import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import { BrowserRouter } from 'react-router-dom';
import App from './App';
import { store } from './app/store';
import { client } from './utils/graphql';
import DateAdapter from '@mui/lab/AdapterDayjs';
import { LocalizationProvider } from '@mui/lab';

ReactDOM.render(
  <React.StrictMode>
    <Provider store={store}>
      <CustomTheme>
        <SnackbarProvider>
          <ApolloProvider client={client}>
            <BrowserRouter>
              <LocalizationProvider dateAdapter={DateAdapter}>
                <App />
              </LocalizationProvider>
            </BrowserRouter>
          </ApolloProvider>
        </SnackbarProvider>
      </CustomTheme>
    </Provider>
  </React.StrictMode>,
  document.getElementById('root'),
);
