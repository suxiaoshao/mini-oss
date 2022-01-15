import { ErrorPage } from 'common';
import { useEffect } from 'react';
import { createSearchParams, Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import { useAppSelector } from '../app/hooks';
import Home from '../pages/Home';
import Login from '../pages/Login';
import Setting from '../pages/Setting';
import AppDrawer from './AppDrawer';

export default function AppRouter(): JSX.Element {
  const auth = useAppSelector((state) => state.auth.value);
  const navigate = useNavigate();
  const { pathname, search, hash } = useLocation();

  useEffect(() => {
    if (auth === null) {
      const url = pathname + search + hash;
      navigate({ pathname: '/login', search: createSearchParams({ from: url }).toString() });
    }
    // 只有 auth 改变时才应该修改路由
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [auth]);
  return (
    <Routes>
      <Route path="/" element={<AppDrawer />}>
        <Route index element={<Home />} />
        <Route path="setting" element={<Setting />} />
        <Route path="*" element={<ErrorPage to={'/'} />} />
      </Route>
      <Route path="/login" element={<Login />} />
    </Routes>
  );
}
