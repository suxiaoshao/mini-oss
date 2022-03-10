import { Box, Card, CardContent, CardHeader, Divider, Typography } from '@mui/material';
import { dayjs } from 'common';
import { useBucketInfoQuery } from 'graphql';
import prettyBytes from 'pretty-bytes';
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
      <Card>
        <CardHeader title="用量概览" />
        <CardContent sx={{ display: 'flex', justifyContent: 'space-between' }}>
          <Box sx={{ display: 'flex', flex: '1 1 0' }}>
            <Box sx={{ flex: '1 1 0' }}>
              <Typography>对象数量</Typography>
              <Typography variant="h4" sx={{ marginTop: (theme) => theme.spacing(1) }}>
                {bucketInfo?.objectCount}
              </Typography>
            </Box>
            <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
          </Box>
          <Box sx={{ display: 'flex', flex: '1 1 0' }}>
            <Box sx={{ flex: '1 1 0' }}>
              <Typography>存储量</Typography>
              <Typography variant="h4" sx={{ marginTop: (theme) => theme.spacing(1) }}>
                {prettyBytes(Number(bucketInfo?.objectSize ?? '0'))}
              </Typography>
            </Box>
            <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
          </Box>
          <Box sx={{ display: 'flex', flex: '1 1 0' }}>
            <Box sx={{ flex: '1 1 0' }}>
              <Typography>本月上传流量</Typography>
              <Typography variant="h4" sx={{ marginTop: (theme) => theme.spacing(1) }}>
                {prettyBytes(Number(bucketInfo?.uploadSize ?? '0'))}
              </Typography>
            </Box>
            <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
          </Box>
          <Box sx={{ display: 'flex', flex: '1 1 0' }}>
            <Box sx={{ flex: '1 1 0' }}>
              <Typography>本月下载流量</Typography>
              <Typography variant="h4" sx={{ marginTop: (theme) => theme.spacing(1) }}>
                {prettyBytes(Number(bucketInfo?.downloadSize ?? '0'))}
              </Typography>
            </Box>
            <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
          </Box>
          <Box sx={{ display: 'flex', flex: '1 1 0' }}>
            <Box sx={{ flex: '1 1 0' }}>
              <Typography>本月请求量</Typography>
              <Typography variant="h4" sx={{ marginTop: (theme) => theme.spacing(1) }}>
                {bucketInfo?.requestCount}
              </Typography>
            </Box>
          </Box>
        </CardContent>
      </Card>
    </Box>
  );
}
