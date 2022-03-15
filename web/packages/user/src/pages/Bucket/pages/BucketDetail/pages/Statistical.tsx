import Dosage from '@/components/Dosage';
import { Box } from '@mui/material';
import { dayjs } from 'common';
import { useBucketInfoQuery } from 'graphql';
import { useMemo } from 'react';
import { useParams } from 'react-router-dom';

export default function Statistical(): JSX.Element {
  const { bucketName = '' } = useParams<{ bucketName: string }>();
  const startTime = useMemo(() => {
    const time = dayjs().date(1).hour(0).minute(0).second(0).millisecond(0).valueOf();
    return time;
  }, []);
  const endTime = useMemo(() => {
    const time = dayjs().valueOf();
    return time;
  }, []);

  const { data: { bucketInfo } = {} } = useBucketInfoQuery({ variables: { data: { bucketName }, startTime, endTime } });
  return (
    <Box
      sx={{
        padding: (theme) => theme.spacing(2),
        display: 'flex',
        flexDirection: 'column',
        flex: '1 1 0',
      }}
    >
      {bucketInfo && <Dosage {...bucketInfo} />}
    </Box>
  );
}
