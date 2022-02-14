import { Delete } from '@mui/icons-material';
import { FormControl, FormLabel, InputBase, IconButton, Button, Box } from '@mui/material';
import { CustomColumnArray, CustomTable, useCustomTable } from 'common';
import { HeaderType } from 'graphql';
import { useMemo } from 'react';
import { Control, useFieldArray, UseFormRegister } from 'react-hook-form';
import { UpdateObjectForm } from './UpdateObjectButton';

export interface HeadersInputProps {
  value: HeaderType[];
  register: UseFormRegister<UpdateObjectForm>;
  control: Control<UpdateObjectForm, object>;
}

export default function HeadersInput({ value, register, control }: HeadersInputProps): JSX.Element {
  const { remove, append } = useFieldArray({ control, name: 'headers' });
  const columns = useMemo<CustomColumnArray<HeaderType>>(
    () => [
      {
        Header: '参数',
        id: 'key',
        accessor: (_, index) => <InputBase {...register(`headers.${index}.key`)} />,
      },
      {
        Header: '值',
        id: 'value',
        accessor: (_, index) => <InputBase {...register(`headers.${index}.value`)} />,
      },
      {
        Header: '操作',
        id: 'action',
        accessor: (_, index) => (
          <IconButton onClick={() => remove(index)}>
            <Delete />
          </IconButton>
        ),
        cellProps: { align: 'center', padding: 'checkbox' },
        headerCellProps: { width: '70px', align: 'center' },
      },
    ],
    [register, remove],
  );

  const tableInstance = useCustomTable({ columns, data: Array.from(value ?? []) });
  return (
    <FormControl required sx={{ marginTop: (theme) => theme.spacing(1), width: '100%' }}>
      <FormLabel>自定义 Headers</FormLabel>
      <CustomTable
        containerProps={{ sx: { height: 300, marginTop: (theme) => theme.spacing(1), overflow: 'hidden' } }}
        tableInstance={tableInstance}
      />
      <Box sx={{ display: 'flex', marginTop: (theme) => theme.spacing(1) }}>
        <Button
          onClick={() => {
            append({ key: '', value: '' });
          }}
          variant="contained"
          sx={{ marginLeft: 'auto' }}
        >
          添加
        </Button>
      </Box>
    </FormControl>
  );
}
