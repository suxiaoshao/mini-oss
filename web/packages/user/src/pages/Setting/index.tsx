import { AppBar, Box, Toolbar, Typography } from '@mui/material';
import UserInfoSetting from './components/UserInfoSetting';
import UserPasswordReset from './components/UserPasswordReset';

export default function Setting(): JSX.Element {
  return (
    <Box sx={{ display: 'flex', width: '100%', height: '100%', flexDirection: 'column' }}>
      <AppBar position="static" color="default">
        <Toolbar>
          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1, display: { xs: 'none', sm: 'block' } }}>
            设置
          </Typography>
        </Toolbar>
      </AppBar>
      <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', flex: '1 1 0' }}>
        <UserInfoSetting />
        <UserPasswordReset />
      </Box>
    </Box>
  );
}
