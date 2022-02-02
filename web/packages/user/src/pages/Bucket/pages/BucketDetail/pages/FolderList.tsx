import { CustomColumnArray, CustomTable, format, useCustomTable, usePage } from 'common';
import { FolderListQuery, useFolderListQuery } from 'graphql';
import { useAppSelector } from '@/app/hooks';
import { useMemo } from 'react';
import { Box, Breadcrumbs, IconButton, Link, Typography } from '@mui/material';
import { createSearchParams, Link as RouterLink, useParams, useSearchParams } from 'react-router-dom';
import parsePath from '@/pages/Bucket/pages/BucketDetail/utils/parsePath';
import { Folder, Refresh } from '@mui/icons-material';
import CreateFolderButton from '@/pages/Bucket/pages/BucketDetail/components/CreateFolderButton';
import AccessFormat from '@/pages/Bucket/components/AccessFormat';
import UploadObjectButton from '@/pages/Bucket/pages/BucketDetail/components/UploadObjectButton';

type FolderTableData = FolderListQuery['folderList']['data'][0];

export default function FolderList(): JSX.Element {
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  // 获取路由数据
  const [searchParams] = useSearchParams();
  const { bucketName = '' } = useParams<{ bucketName: string }>();
  const path = useMemo(() => searchParams.get('path'), [searchParams]) ?? '/';
  const pathList = useMemo(() => parsePath(path), [path]);

  const { limit, offset } = usePage({});
  const { data: { folderList } = {}, refetch } = useFolderListQuery({
    variables: { data: { limit, offset, auth, bucketName, path } },
  });
  const columns = useMemo<CustomColumnArray<FolderTableData>>(
    () => [
      {
        Header: '名字',
        id: 'name',
        accessor: (row) => {
          if (row.__typename === 'FolderInfo') {
            return (
              <Link
                component={RouterLink}
                to={{ search: createSearchParams({ path: row.path }).toString() }}
                sx={{ display: 'flex', lineHeight: 1.8 }}
              >
                <Folder sx={{ marginRight: (theme) => theme.spacing(0.5) }} />
                {row.folderName}/
              </Link>
            );
          } else {
            return row.filename;
          }
        },
      },
      {
        Header: '访问权限',
        id: 'access',
        accessor: ({ access }) => <AccessFormat access={access} />,
      },
      {
        Header: '创建时间',
        id: 'createTime',
        accessor: ({ createTime }) => format(createTime),
      },
      {
        Header: '更新时间',
        id: 'updateTime',
        accessor: ({ updateTime }) => format(updateTime),
      },
    ],
    [],
  );
  const tableInstance = useCustomTable({ columns, data: folderList?.data ?? [] });
  return (
    <Box
      sx={{
        padding: (theme) => theme.spacing(2),
        display: 'flex',
        flexDirection: 'column',
        flex: '0 0 1',
      }}
    >
      <Breadcrumbs>
        <Link
          component={RouterLink}
          to={{
            search: createSearchParams({}).toString(),
          }}
          underline="hover"
          color="inherit"
        >
          {bucketName}
        </Link>
        {pathList.slice(0, -1).map((value) => (
          <Link
            component={RouterLink}
            to={{ search: createSearchParams({ path: value.path }).toString() }}
            key={value.path}
          >
            {value.folderName}
          </Link>
        ))}
        {pathList.at(-1) && <Typography color="text.primary">{pathList.at(-1)?.folderName}</Typography>}
      </Breadcrumbs>
      <Box
        sx={{
          flex: '0 0 auto',
          marginBottom: (theme) => theme.spacing(2),
          marginTop: (theme) => theme.spacing(2),
          display: 'flex',
        }}
      >
        <UploadObjectButton reFetch={refetch} path={path} bucketName={bucketName} />
        <CreateFolderButton reFetch={refetch} fatherPath={path} bucketName={bucketName} />
        <IconButton sx={{ marginLeft: 'auto' }} onClick={() => refetch()}>
          <Refresh />
        </IconButton>
      </Box>
      <CustomTable tableInstance={tableInstance} />
    </Box>
  );
}
