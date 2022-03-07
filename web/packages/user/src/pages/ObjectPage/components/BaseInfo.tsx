import AccessFormat from '@/components/AccessFormat';
import { getPath } from '@/utils/axios';
import { ContentCopy } from '@mui/icons-material';
import { Card, CardHeader, CardContent, IconButton } from '@mui/material';
import { format, useSnackbar } from 'common';
import { GetObjectQuery } from 'graphql';
import prettyBytes from 'pretty-bytes';
import { useMemo } from 'react';
import InfoItem from './InfoItem';

export interface BaseInfoProps {
  objectInfo: GetObjectQuery['objectInfo'];
}

export default function BaseInfo({ objectInfo }: BaseInfoProps): JSX.Element {
  const url = useMemo(
    () => getPath(objectInfo.bucketName, objectInfo.path, objectInfo.filename),
    [objectInfo.bucketName, objectInfo.filename, objectInfo.path],
  );
  // 提示插件
  const { enqueueSnackbar } = useSnackbar();
  return (
    <Card>
      <CardHeader title="基本信息" />
      <CardContent>
        <InfoItem label="对象名称" value={objectInfo.filename} />
        <InfoItem label="对象大小" value={prettyBytes(Number(objectInfo.size))} />
        <InfoItem label="访问权限" value={<AccessFormat access={objectInfo.access} />} />
        <InfoItem label="修改时间" value={format(objectInfo.updateTime)} />
        <InfoItem label="Etag" value={`"${objectInfo.blake3}"`} />
        <InfoItem
          last
          label="对象地址"
          value={
            <>
              {decodeURI(url)}
              <IconButton
                onClick={async () => {
                  await window.navigator.clipboard.writeText(decodeURI(url));
                  enqueueSnackbar('复制到剪切板成功', { variant: 'success' });
                }}
              >
                <ContentCopy />
              </IconButton>
            </>
          }
        />
      </CardContent>
    </Card>
  );
}
