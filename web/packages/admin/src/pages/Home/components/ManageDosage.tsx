import { Card, CardHeader, CardContent, Box, Divider, CardProps, Alert } from '@mui/material';
import { InfoWithNumber } from 'common';
import prettyBytes from 'pretty-bytes';

export interface DosageProps extends CardProps {
  objectSize: string;
  userCount: number;
  uploadSize: string;
  downloadSize: string;
  requestCount: number;
}

export default function ManageDosage({
  objectSize,
  userCount,
  uploadSize,
  downloadSize,
  requestCount,
  ...props
}: DosageProps): JSX.Element {
  return (
    <Card {...props}>
      <Alert severity="success">用户数量和存储量为当前数据, 其余项为时间范围内数据之和</Alert>
      <CardHeader title="用量概览" />
      <CardContent sx={{ display: 'flex', justifyContent: 'space-between' }}>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="用户数量" value={userCount} />
          <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
        </Box>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="存储量" value={prettyBytes(Number(objectSize))} />
          <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
        </Box>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="上传流量" value={prettyBytes(Number(uploadSize))} />
          <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
        </Box>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="下载流量" value={prettyBytes(Number(downloadSize))} />
          <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
        </Box>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="请求量" value={requestCount} />
        </Box>
      </CardContent>
    </Card>
  );
}
