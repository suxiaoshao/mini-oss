import { AppBar, Box, Toolbar, Typography } from '@mui/material';

export default function Home(): JSX.Element {
  return (
    <Box sx={{ color: '#fff', display: 'flex', width: '100%', height: '100%', flexDirection: 'column' }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1, display: { xs: 'none', sm: 'block' } }}>
            概览
          </Typography>
        </Toolbar>
      </AppBar>
      home
    </Box>
  );
}
