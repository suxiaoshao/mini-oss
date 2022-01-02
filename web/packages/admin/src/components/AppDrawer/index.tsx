import { Home } from '@mui/icons-material';
import { Box, Drawer, List } from '@mui/material';
import { Outlet } from 'react-router-dom';
import ListItemRoute from './ListItemRoute';

export default function AppDrawer(): JSX.Element {
  const width = 250;
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
        </List>
      </Drawer>

      <Box sx={{ flex: '1 1 0', maxWidth: `calc(100% - ${width}px)` }}>
        <Outlet />
      </Box>
    </Box>
  );
}
