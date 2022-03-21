import AccessFormat from '@/components/AccessFormat';
import InfoItem from '@/pages/ObjectPage/components/InfoItem';
import { getPath } from '@/utils/axios';
import { ContentCopy } from '@mui/icons-material';
import { Card, CardHeader, CardContent, IconButton } from '@mui/material';
import { format, useSnackbar } from 'common';
import { BucketInfoQuery } from 'graphql';
import { useMemo } from 'react';

export type BucketBaseInfoProps = Pick<BucketInfoQuery['bucketInfo'], 'name' | 'createTime' | 'updateTime' | 'access'>;

export default function BaseInfo({ name, createTime, updateTime, access }: BucketBaseInfoProps): JSX.Element {
  const url = useMemo(() => getPath(name, '', ''), [name]);
  // 提示插件
  const { enqueueSnackbar } = useSnackbar();
  return (
    <Card sx={{ marginTop: (theme) => theme.spacing(2) }}>
      <CardHeader title="基本信息" />
      <CardContent>
        <InfoItem label="存储桶名称" value={name} />
        <InfoItem label="访问权限" value={<AccessFormat access={access} />} />
        <InfoItem label="创建时间" value={format(createTime)} />
        <InfoItem label="修改时间" value={format(updateTime)} />
        <InfoItem
          last
          label="访问域名"
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
