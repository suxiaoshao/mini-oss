import InfoWithNumber from '@/components/Dosage/InfoWithNumber';
import { Card, CardHeader, CardContent, Box, Divider, CardProps } from '@mui/material';
import prettyBytes from 'pretty-bytes';

export interface DosageProps extends CardProps {
  objectSize: string;
  objectCount: number;
  uploadSize: string;
  downloadSize: string;
  requestCount: number;
}

export default function BucketDosage({
  objectSize,
  objectCount,
  uploadSize,
  downloadSize,
  requestCount,
  ...props
}: DosageProps): JSX.Element {
  return (
    <Card {...props}>
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
