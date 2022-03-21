import Dosage from '@/components/Dosage';
import { Box } from '@mui/material';
import { dayjs } from 'common';
import { useBucketInfoQuery } from 'graphql';
import { useState } from 'react';
import { useParams } from 'react-router-dom';
import BucketBaseInfo from './components/BucketBaseInfo';
import BucketChart from './components/BucketChart';
import DurationSelect from './components/DurationSelect';

export default function Statistical(): JSX.Element {
  const { bucketName = '' } = useParams<{ bucketName: string }>();
  const [startTime, setStartTime] = useState(dayjs().startOf('day').valueOf());
  const [endTime, setEndTime] = useState(dayjs().valueOf());

  const { data: { bucketInfo } = {} } = useBucketInfoQuery({ variables: { data: { bucketName }, startTime, endTime } });
  return (
    <Box
      sx={{
        padding: (theme) => theme.spacing(2),
        flex: '1 1 0',
        overflow: 'auto',
      }}
    >
      <DurationSelect setStartTime={setStartTime} setEndTime={setEndTime} />
      {bucketInfo && <Dosage sx={{ marginTop: (theme) => theme.spacing(2) }} {...bucketInfo} />}
      {bucketInfo && <BucketBaseInfo {...bucketInfo} />}
      {bucketInfo && <BucketChart {...bucketInfo} />}
    </Box>
  );
}
