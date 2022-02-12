import { Delete } from '@mui/icons-material';
import { Button, Box, Input, IconButton, FormHelperText } from '@mui/material';
import { CustomColumnArray, CustomTable, useCustomTable } from 'common';
import prettyBytes from 'pretty-bytes';
import React, { ChangeEvent, useMemo } from 'react';
import { ReactChild } from 'react';

export interface ControllerFilesProps {
  label: ReactChild;
  name: string;
  value: File[];
  /** 添加文件 */
  append: (file: File) => void;
  remove: (index?: number | number[]) => void;
}

function ControllerFiles(
  { label, name, value, append, remove }: ControllerFilesProps,
  ref?: React.Ref<never>,
): JSX.Element {
  const columns = useMemo<CustomColumnArray<FileList[0]>>(
    () => [
      {
        Header: '名字',
        id: 'name',
        accessor: (row) => row.name,
      },
      {
        Header: '文件大小',
        id: 'size',
        accessor: (row) => prettyBytes(row.size),
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
    [remove],
  );

  const tableInstance = useCustomTable({ columns, data: Array.from(value ?? []) });
  return (
    <>
      <Box sx={{ display: 'flex' }} component="label" htmlFor="contained-button-file">
        <Input
          name={name}
          onChange={(e) => {
            const event = e as ChangeEvent<HTMLInputElement>;
            Array.from(event.target.files ?? []).forEach((file) => {
              append(file);
            });
          }}
          sx={{ display: 'none' }}
          ref={ref}
          id="contained-button-file"
          type="file"
          inputProps={{ multiple: true }}
        />
        <Button variant="contained" component="span">
          {label}
        </Button>
      </Box>
      <FormHelperText error>{value.length === 0 && '不能上传空文件'}</FormHelperText>
      {value.length > 0 && (
        <CustomTable
          containerProps={{ sx: { overflow: 'auto', maxHeight: 400, marginTop: (theme) => theme.spacing(1) } }}
          tableInstance={tableInstance}
        />
      )}
    </>
  );
}
export default React.forwardRef(ControllerFiles) as typeof ControllerFiles;
