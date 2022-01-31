import { Refresh } from '@mui/icons-material';
import { AppBar, Box, IconButton, Toolbar, Typography } from '@mui/material';
import { CustomColumnArray, CustomTable, format, TableActions, useCustomTable, usePage } from 'common';
import { UserListQuery, useUserDeleteMutation, useUserListQuery } from 'graphql';
import { useMemo } from 'react';
import { useAppSelector } from '@/app/hooks';
import CreateUserButton from './components/CreateUserButton';
import UpdateUserAction from './components/UpdateUserAction';

export default function Users(): JSX.Element {
  const { limit, offset } = usePage({});
  const auth = useAppSelector((state) => state.auth.value);
  const { data, refetch } = useUserListQuery({
    variables: { data: { limit, offset, auth: auth ?? '' } },
    skip: auth === null,
  });
  const [deleteUser] = useUserDeleteMutation();
  const columns = useMemo<CustomColumnArray<UserListQuery['userList']['data'][0]>>(
    () => [
      {
        Header: '名字',
        id: 'name',
        accessor: 'name',
      },
      {
        Header: '描述',
        id: 'description',
        accessor: ({ description }) => description ?? '-',
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
        accessor: ({ name, description }) => (
          <TableActions>
            {(onClose) => [
              {
                text: '删除',
                onClick: async () => {
                  await deleteUser({ variables: { data: { auth: auth ?? '', name } } });
                  onClose();
                  await refetch();
                },
              },
              <UpdateUserAction
                description={description ?? undefined}
                name={name}
                refetch={refetch}
                key={2}
                menuClose={onClose}
              />,
            ]}
          </TableActions>
        ),
        cellProps: { padding: 'none' },
      },
    ],
    [auth, deleteUser, refetch],
  );

  const tableInstance = useCustomTable({ columns, data: data?.userList.data ?? [] });

  return (
    <Box
      sx={{
        width: '100',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
      }}
    >
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1, display: { xs: 'none', sm: 'block' } }}>
            用户列表
          </Typography>
        </Toolbar>
      </AppBar>
      <Box
        sx={{
          padding: (theme) => theme.spacing(2),
          display: 'flex',
          flexDirection: 'column',
          flex: '1 1 0',
        }}
      >
        <Box sx={{ flex: '0 0 auto', marginBottom: (theme) => theme.spacing(2), display: 'flex' }}>
          <CreateUserButton refetch={refetch} />
          <IconButton sx={{ marginLeft: 'auto' }} onClick={() => refetch()}>
            <Refresh />
          </IconButton>
        </Box>
        <CustomTable tableInstance={tableInstance} />
      </Box>
    </Box>
  );
}
