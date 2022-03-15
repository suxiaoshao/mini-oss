import { Card, CardHeader, CardContent, Box, Divider } from '@mui/material';
import prettyBytes from 'pretty-bytes';
import InfoWithNumber from './InfoWithNumber';

export interface DosageProps {
  objectSize: string;
  objectCount: number;
  uploadSize: string;
  downloadSize: string;
  requestCount: number;
}

export default function Dosage({
  objectSize,
  objectCount,
  uploadSize,
  downloadSize,
  requestCount,
}: DosageProps): JSX.Element {
  return (
    <Card>
      <CardHeader title="用量概览" />
      <CardContent sx={{ display: 'flex', justifyContent: 'space-between' }}>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="对象数量" value={objectCount} />
          <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
        </Box>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="存储量" value={prettyBytes(Number(objectSize))} />
          <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
        </Box>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="本月上传流量" value={prettyBytes(Number(uploadSize))} />
          <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
        </Box>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="本月下载流量" value={prettyBytes(Number(downloadSize))} />
          <Divider orientation="vertical" flexItem sx={{ marginRight: (theme) => theme.spacing(2) }} />
        </Box>
        <Box sx={{ display: 'flex', flex: '1 1 0' }}>
          <InfoWithNumber name="本月请求量" value={requestCount} />
        </Box>
      </CardContent>
    </Card>
  );
}
