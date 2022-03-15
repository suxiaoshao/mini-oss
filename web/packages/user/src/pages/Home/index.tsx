import Dosage from '@/components/Dosage';
import { AppBar, Box, Toolbar, Typography } from '@mui/material';
import { dayjs } from 'common';
import { useUserStatQuery } from 'graphql';
import { useMemo } from 'react';

export default function Home(): JSX.Element {
  const startTime = useMemo(() => {
    const time = dayjs().date(1).hour(0).minute(0).second(0).millisecond(0).valueOf();
    return time;
  }, []);
  const endTime = useMemo(() => {
    const time = dayjs().valueOf();
    return time;
  }, []);

  const { data: { selfUserInfo } = {} } = useUserStatQuery({ variables: { startTime, endTime } });
  return (
    <Box sx={{ display: 'flex', width: '100%', height: '100%', flexDirection: 'column' }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1, display: { xs: 'none', sm: 'block' } }}>
            概览
          </Typography>
        </Toolbar>
      </AppBar>
      <Box sx={{ flex: '1 1 0', padding: (theme) => theme.spacing(2) }}>
        {selfUserInfo && <Dosage {...selfUserInfo} />}
      </Box>
    </Box>
  );
}
