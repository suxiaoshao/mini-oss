import { AllInbox, Home, Logout, Settings } from '@mui/icons-material';
import { Box, Divider, Drawer, List, ListItemButton, ListItemIcon, ListItemText } from '@mui/material';
import { Outlet } from 'react-router-dom';
import { useAppDispatch } from '../../app/hooks';
import { logout } from '../../features/auth/authSlice';
import ListItemRoute from './ListItemRoute';

export default function AppDrawer(): JSX.Element {
  const width = 250;
  const dispatch = useAppDispatch();
  return (
    <Box sx={{ display: 'flex', flexDirection: 'row', height: '100%' }}>
      <Drawer
        sx={{
          flex: `0 0 ${width}px`,
        }}
        variant="persistent"
        anchor="left"
        open
      >
        <List sx={{ width }}>
          <ListItemRoute toPath="/" matchPaths={['/', '']} text="首页" icon={<Home />} />
          <ListItemRoute toPath="/bucket" matchPaths={['/bucket']} text="存储桶" icon={<AllInbox />} />
          <ListItemRoute toPath="/setting" matchPaths={['/setting']} text="设置" icon={<Settings />} />
        </List>
        <Divider />
        <List sx={{ width }}>
          <ListItemButton
            onClick={() => {
              dispatch(logout());
            }}
          >
            <ListItemIcon>
              <Logout />
            </ListItemIcon>
            <ListItemText>登出</ListItemText>
          </ListItemButton>
        </List>
      </Drawer>

      <Box sx={{ flex: '1 1 0', maxWidth: `calc(100% - ${width}px)` }}>
        <Outlet />
      </Box>
    </Box>
  );
}
