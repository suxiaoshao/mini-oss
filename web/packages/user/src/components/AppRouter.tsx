import { useEffect } from 'react';
import { Route, Routes, useNavigate } from 'react-router-dom';
import { useAppSelector } from '../app/hooks';
import Home from '../pages/Home';
import Login from '../pages/Login';
import AppDrawer from './AppDrawer';

export default function AppRouter(): JSX.Element {
  const auth = useAppSelector((state) => state.auth.value);
  const navigate = useNavigate();

  useEffect(() => {
    if (auth === null) {
      navigate('/login');
    }
    // 只有 auth 改变时才应该修改路由
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [auth]);
  return (
    <Routes>
      <Route path="/" element={<AppDrawer />}>
        <Route index element={<Home />} />
      </Route>
      <Route path="/login" element={<Login />} />
    </Routes>
  );
}
