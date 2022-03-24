import { AppBar, Box, Toolbar, Typography } from '@mui/material';
import { Charts, dayjs, DurationSelect } from 'common';
import { useManageDetailQuery } from 'graphql';
import { useState } from 'react';
import ManageDosage from './components/ManageDosage';

export default function Home(): JSX.Element {
  const [startTime, setStartTime] = useState(dayjs().startOf('day').valueOf());
  const [endTime, setEndTime] = useState(dayjs().valueOf());

  const { data: { manageDetail } = {} } = useManageDetailQuery({ variables: { startTime, endTime } });
  return (
    <Box sx={{ color: '#fff', display: 'flex', width: '100%', height: '100%', flexDirection: 'column' }}>
      <AppBar position="static" color="default">
        <Toolbar>
          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1, display: { xs: 'none', sm: 'block' } }}>
            概览
          </Typography>
        </Toolbar>
      </AppBar>
      <Box sx={{ flex: '1 1 0', padding: (theme) => theme.spacing(2), overflow: 'auto' }}>
        <DurationSelect setStartTime={setStartTime} setEndTime={setEndTime} />
        {manageDetail && (
          <>
            <ManageDosage sx={{ marginTop: (theme) => theme.spacing(2) }} {...manageDetail} />
            <Charts {...manageDetail} />
          </>
        )}
      </Box>
    </Box>
  );
}
