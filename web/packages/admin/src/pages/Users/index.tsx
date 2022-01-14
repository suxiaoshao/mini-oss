import { Box } from '@mui/material';
import { CustomTable, format, TableActions, usePage } from 'common';
import { UserListQuery, useUserDeleteMutation, useUserListQuery } from 'graphql';
import { useMemo } from 'react';
import { Column, useTable } from 'react-table';
import { useAppSelector } from '../../app/hooks';
import CreateUserFab from './components/CreateUserFab';
import UpdateUserAction from './components/UpdateUserAction';
export default function Users(): JSX.Element {
  const { limit, offset } = usePage({});
  const auth = useAppSelector((state) => state.auth.value);
  const { data, refetch } = useUserListQuery({ variables: { limit, offset, auth: auth ?? '' }, skip: auth === null });
  const [deleteUser] = useUserDeleteMutation();
  const columns = useMemo<ReadonlyArray<Column<UserListQuery['userList']['data'][0]>>>(
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
                  await deleteUser({ variables: { auth: auth ?? '', name } });
                  onClose();
                  refetch();
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
        width: 50,
      },
    ],
    [auth, deleteUser, refetch],
  );

  const tableInstance = useTable({ columns, data: data?.userList.data ?? [] });

  return (
    <Box sx={{ width: '100', height: '100%', padding: (theme) => theme.spacing(2) }}>
      <CustomTable tableInstance={tableInstance} />
      <CreateUserFab refetch={refetch} />
    </Box>
  );
}
