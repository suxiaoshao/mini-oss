import { AppBar, Box, IconButton, Tab, Tabs, Typography } from '@mui/material';
import { Link as RouterLink, Outlet, useLocation, useNavigate, useParams } from 'react-router-dom';
import { ArrowBack } from '@mui/icons-material';
import { useMemo } from 'react';

export default function BucketDetail(): JSX.Element {
  // 获取路由数据
  const { bucketName = '' } = useParams<{ bucketName: string }>();
  const location = useLocation();
  const navigate = useNavigate();
  const value = useMemo<'detail' | 'fileList'>(() => {
    const re = /fileList$/;
    return location.pathname.match(re) === null ? 'detail' : 'fileList';
  }, [location]);
  return (
    <Box sx={{ display: 'flex', width: '100%', height: '100%', flexDirection: 'column' }}>
      <AppBar
        position="static"
        color="default"
        sx={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'flex-start',
          padding: (theme) => theme.spacing(1),
          paddingBottom: 0,
        }}
      >
        <Box sx={{ width: '100%', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <Box>
            <IconButton to={'/bucket/list'} component={RouterLink}>
              <ArrowBack />
            </IconButton>
            返回桶列表
          </Box>
          <Typography variant="h6" noWrap component="div" sx={{ display: { xs: 'none', sm: 'block' } }}>
            {bucketName}
          </Typography>
          <Box sx={{ visibility: 'hidden' }}>
            <IconButton to={'/bucket/list'} component={RouterLink}>
              <ArrowBack />
            </IconButton>
            返回桶列表
          </Box>
        </Box>
        <Tabs
          sx={{ width: '100%' }}
          centered
          value={value}
          onChange={(_, newValue: 'detail' | 'fileList') => {
            if (newValue === 'fileList') {
              navigate(`${location.pathname}/fileList`);
            } else {
              navigate(location.pathname.split('/').slice(0, -1).join('/'));
            }
          }}
        >
          <Tab label="概览" value={'detail'} />
          <Tab label="文件列表" value={'fileList'} />
        </Tabs>
      </AppBar>
      <Outlet />
    </Box>
  );
}
