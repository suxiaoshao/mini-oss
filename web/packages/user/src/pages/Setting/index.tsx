import { Box } from '@mui/material';
import UserInfoSetting from './components/UserInfoSetting';
import UserPasswordReset from './components/UserPasswordReset';

export default function Setting(): JSX.Element {
  return (
    <Box sx={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
      <UserInfoSetting />
      <UserPasswordReset />
    </Box>
  );
}
