import { useAppSelector } from '@/app/hooks';
import { Refresh } from '@mui/icons-material';
import { Box, IconButton } from '@mui/material';
import { useGetObjectQuery } from 'graphql';
import { useEffect } from 'react';
import { useParams, useSearchParams, useNavigate } from 'react-router-dom';
import BaseInfo from './components/BaseInfo';
import HeadersInfo from './components/HeadersInfo';
import ObjectHeader from './components/ObjectHeader';
import UpdateObjectButton from './components/UpdateObjectButton';

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
  // 获取数据
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const { data: { objectInfo } = {}, refetch } = useGetObjectQuery({
    variables: { data: { bucketName, path, filename } },
    skip: auth === '',
  });

  return (
    <Box sx={{ display: 'flex', width: '100%', height: '100%', flexDirection: 'column' }}>
      <ObjectHeader bucketName={bucketName} path={path} filename={filename} />
      <Box
        sx={{
          padding: (theme) => theme.spacing(2),
          display: 'flex',
          flexDirection: 'column',
          flex: '1 1 0',
        }}
      >
        <Box
          sx={{
            flex: '0 0 auto',
            marginBottom: (theme) => theme.spacing(2),
            marginTop: (theme) => theme.spacing(2),
            display: 'flex',
          }}
        >
          {objectInfo && <UpdateObjectButton reFetch={refetch} objectInfo={objectInfo} />}
          <IconButton sx={{ marginLeft: 'auto' }} onClick={() => refetch()}>
            <Refresh />
          </IconButton>
        </Box>
        {objectInfo && (
          <>
            <BaseInfo objectInfo={objectInfo} />
            <HeadersInfo value={objectInfo.headers} />
          </>
        )}
      </Box>
    </Box>
  );
}
