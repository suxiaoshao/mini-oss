import parsePath from '@/pages/Bucket/pages/BucketDetail/utils/parsePath';
import { AppBar, Typography, Breadcrumbs, Link } from '@mui/material';
import { useMemo } from 'react';
import { createSearchParams, Link as RouterLink } from 'react-router-dom';

export interface ObjectHeaderProps {
  bucketName: string;
  path: string;
  filename: string;
}

export default function ObjectHeader({ bucketName, path, filename }: ObjectHeaderProps): JSX.Element {
  const pathList = useMemo(() => parsePath(path), [path]);
  return (
    <AppBar
      position="static"
      color="default"
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
  );
}
