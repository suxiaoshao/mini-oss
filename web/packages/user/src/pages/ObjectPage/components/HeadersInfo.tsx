import { Card, CardHeader } from '@mui/material';
import { CustomColumnArray, CustomTable, useCustomTable } from 'common';
import { GetObjectQuery } from 'graphql';
import { useMemo } from 'react';
type Header = GetObjectQuery['objectInfo']['headers'][0];
export default function HeadersInfo({ value }: { value: Header[] }): JSX.Element {
  const columns = useMemo<CustomColumnArray<Header>>(
    () => [
      {
        Header: '参数',
        id: 'key',
        accessor: 'key',
      },
      {
        Header: '值',
        id: 'value',
        accessor: 'value',
      },
    ],
    [],
  );
  const tableInstance = useCustomTable({ columns, data: value });
  return (
    <Card sx={{ marginTop: (theme) => theme.spacing(2), flex: '1 1 0' }}>
      <CardHeader title="自定义 headers" />
      <CustomTable containerProps={{ sx: { height: '100%' } }} tableInstance={tableInstance} />
    </Card>
  );
}
