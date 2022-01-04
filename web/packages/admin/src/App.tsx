import { useSnackbarInit } from 'common';
import AppRouter from './components/AppRouter';

function App(): JSX.Element {
  useSnackbarInit();
  return <AppRouter />;
}

export default App;
