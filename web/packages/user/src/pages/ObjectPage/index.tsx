import { useAppSelector } from '@/app/hooks';
import { getPath } from '@/utils/axios';
import { ContentCopy } from '@mui/icons-material';
import { AppBar, Box, Breadcrumbs, Card, CardContent, CardHeader, IconButton, Link, Typography } from '@mui/material';
import { format, useSnackbar } from 'common';
import { useGetObjectQuery } from 'graphql';
import prettyBytes from 'pretty-bytes';
import { ReactChild, useEffect, useMemo } from 'react';
import { createSearchParams, useParams, Link as RouterLink, useSearchParams, useNavigate } from 'react-router-dom';
import parsePath from '../Bucket/pages/BucketDetail/utils/parsePath';

export default function BucketList(): JSX.Element {
  // 路由数据处理
  const { bucketName = '' } = useParams<{ bucketName: string }>();
  const [searchParams] = useSearchParams();
  const path = searchParams.get('path') ?? '';
  const filename = searchParams.get('filename') ?? '';
  const navigate = useNavigate();
  useEffect(() => {
    if (filename === '' || path === '' || bucketName === '') {
      navigate(`/bucket/detail/${bucketName}`);
    }
  }, [path, filename, navigate, bucketName]);
  const pathList = useMemo(() => parsePath(path), [path]);
  // 获取数据
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const { data: { objectInfo } = {} } = useGetObjectQuery({
    variables: { data: { auth, bucketName, path, filename } },
    skip: auth === '',
  });
  // 提示插件
  const { enqueueSnackbar } = useSnackbar();

  return (
    <Box sx={{ display: 'flex', width: '100%', height: '100%', flexDirection: 'column' }}>
      <AppBar
        position="static"
        sx={{
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          padding: (theme) => theme.spacing(1),
        }}
      >
        <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1, display: { xs: 'none', sm: 'block' } }}>
          对象详情
        </Typography>
        <Breadcrumbs sx={{ width: '100%' }}>
          <Link
            component={RouterLink}
            to={{
              pathname: `/bucket/detail/${bucketName}/fileList`,
              search: createSearchParams({}).toString(),
            }}
            underline="hover"
            color="inherit"
          >
            {bucketName}
          </Link>
          {pathList.map((value) => (
            <Link
              component={RouterLink}
              to={{
                pathname: `/bucket/detail/${bucketName}/fileList`,
                search: createSearchParams({ path: value.path }).toString(),
              }}
              key={value.path}
            >
              {value.folderName}
            </Link>
          ))}
          <Typography color="text.primary">{filename}</Typography>
        </Breadcrumbs>
      </AppBar>
      <Box
        sx={{
          padding: (theme) => theme.spacing(2),
          display: 'flex',
          flexDirection: 'column',
          flex: '1 1 0',
        }}
      >
        {objectInfo && (
          <Card>
            <CardHeader title="基本信息" />
            <CardContent>
              <InfoItem label="对象名称" value={objectInfo.filename} />
              <InfoItem label="对象大小" value={prettyBytes(objectInfo.size)} />
              <InfoItem label="修改时间" value={format(objectInfo.updateTime)} />
              <InfoItem label="Etag" value={`"${objectInfo.blake3}"`} />
              <InfoItem
                last
                label="对象地址"
                value={
                  <>
                    {getPath(objectInfo.bucketName, path, filename)}
                    <IconButton
                      onClick={async () => {
                        await window.navigator.clipboard.writeText(getPath(objectInfo.bucketName, path, filename));
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
        )}
      </Box>
    </Box>
  );
}

function InfoItem({ label, value, last }: { label: ReactChild; value: ReactChild; last?: boolean }): JSX.Element {
  return (
    <Box sx={{ display: 'flex', alignItems: 'center', marginBottom: (theme) => (last ? 0 : theme.spacing(2)) }}>
      <Typography sx={{ color: (theme) => theme.palette.text.secondary, width: (theme) => theme.spacing(13) }}>
        {label}
      </Typography>
      <Typography>{value}</Typography>
    </Box>
  );
}
