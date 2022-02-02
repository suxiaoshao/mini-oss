import { updateUserInfo } from '@/features/userInfo/userInfoSlice';
import BucketList from '@/pages/Bucket/pages/BucketList';
import { ErrorPage } from 'common';
import { useEffect } from 'react';
import { createSearchParams, Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import { useAppDispatch, useAppSelector } from '@/app/hooks';
import Home from '../pages/Home';
import Login from '../pages/Login';
import Setting from '../pages/Setting';
import AppDrawer from './AppDrawer';
import BucketDetail from '@/pages/Bucket/pages/BucketDetail';
import FolderList from '@/pages/Bucket/pages/BucketDetail/pages/FolderList';
import Statistical from '@/pages/Bucket/pages/BucketDetail/pages/Statistical';

export default function AppRouter(): JSX.Element {
  const auth = useAppSelector((state) => state.auth.value);
  const navigate = useNavigate();
  const { pathname, search, hash } = useLocation();
  const dispatch = useAppDispatch();

  useEffect(() => {
    if (auth === null) {
      const url = pathname + search + hash;
      if (pathname !== '/login') {
        navigate({ pathname: '/login', search: createSearchParams({ from: url }).toString() });
      }
    } else {
      dispatch(updateUserInfo(auth));
    }
    // 只有 auth 改变时才应该修改路由
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [auth, dispatch]);
  return (
    <Routes>
      <Route path="/" element={<AppDrawer />}>
        <Route index element={<Home />} />
        <Route path="bucket">
          <Route path="list" element={<BucketList />} />
          <Route path="detail/:bucketName" element={<BucketDetail />}>
            <Route index element={<Statistical />} />
            <Route path="fileList" element={<FolderList />} />
          </Route>
        </Route>
        <Route path="setting" element={<Setting />} />
        <Route path="*" element={<ErrorPage to={'/'} />} />
      </Route>
      <Route path="/login" element={<Login />} />
    </Routes>
  );
}
