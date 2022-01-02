import { Home } from '@mui/icons-material';
import { Box, Drawer, List } from '@mui/material';
import ListItemRoute from './ListItemRoute';

export interface AppDrawerProps {
  children?: React.ReactChild;
}
export default function AppDrawer({ children }: AppDrawerProps): JSX.Element {
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

      <Box sx={{ flex: '1 1 0', maxWidth: `calc(100% - ${width}px)` }}>{children}</Box>
    </Box>
  );
}
