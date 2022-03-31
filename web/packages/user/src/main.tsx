import { ApolloProvider } from '@apollo/client';
import { SnackbarProvider, CustomTheme } from 'common';
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux';
import { BrowserRouter } from 'react-router-dom';
import App from './App';
import { store } from './app/store';
import { client } from './utils/graphql';
import DateAdapter from '@mui/lab/AdapterDayjs';
import { LocalizationProvider } from '@mui/lab';

createRoot(document.getElementById('root') ?? document.body).render(
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
  </Provider>,
);
