import { BrowserRouter } from 'react-router-dom';
import CustomTheme from '../../../common/src/CustomTheme';
import AppDrawer from './components/AppDrawer';
import AppRouter from './components/AppRouter';

function App(): JSX.Element {
  return (
    <CustomTheme>
      <BrowserRouter>
        <AppDrawer>
          <AppRouter />
        </AppDrawer>
      </BrowserRouter>
    </CustomTheme>
  );
}

export default App;
