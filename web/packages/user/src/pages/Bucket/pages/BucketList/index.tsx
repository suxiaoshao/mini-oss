import { useAppSelector } from '@/app/hooks';
import { Refresh } from '@mui/icons-material';
import { Box, IconButton } from '@mui/material';
import { CustomColumnArray, CustomTable, format, useCustomTable, usePage, TableActions } from 'common';
import { BucketListQuery, useBucketListQuery, useDeleteBucketMutation } from 'graphql';
import { useMemo } from 'react';
import AccessFormat from '../../components/AccessFormat';
import CreateBucketButton from './components/CreateBucketButton';
import UpdateBucketAction from './components/UpdateBucketAction';

export default function BucketList(): JSX.Element {
  const { limit, offset } = usePage({});
  const auth = useAppSelector((state) => state.auth.value);
  const { data, refetch } = useBucketListQuery({ variables: { data: { auth: auth ?? '', limit, offset } } });
  const [deleteBucket] = useDeleteBucketMutation();
  const columns = useMemo<CustomColumnArray<BucketListQuery['bucketList']['data'][0]>>(
    () => [
      {
        Header: '名字',
        id: 'name',
        accessor: 'name',
      },
      {
        Header: '访问',
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
      {
        Header: '操作',
        id: 'action',
        accessor: ({ name, access }) => (
          <TableActions>
            {(onClose) => [
              {
                text: '删除',
                onClick: async () => {
                  await deleteBucket({ variables: { data: { auth: auth ?? '', name } } });
                  onClose();
                  refetch();
                },
              },
              <UpdateBucketAction key={2} refetch={refetch} menuClose={onClose} name={name} access={access} />,
            ]}
          </TableActions>
        ),
      },
    ],
    [auth, deleteBucket, refetch],
  );
  const tableInstance = useCustomTable({ columns, data: data?.bucketList.data ?? [] });
  return (
    <Box
      sx={{
        width: '100',
        height: '100%',
        padding: (theme) => theme.spacing(2),
        display: 'flex',
        flexDirection: 'column',
      }}
    >
      <Box sx={{ flex: '0 0 auto', marginBottom: (theme) => theme.spacing(2), display: 'flex' }}>
        <CreateBucketButton refetch={refetch} />
        <IconButton sx={{ marginLeft: 'auto' }} onClick={() => refetch()}>
          <Refresh />
        </IconButton>
      </Box>
      <CustomTable tableInstance={tableInstance} />
    </Box>
  );
}
