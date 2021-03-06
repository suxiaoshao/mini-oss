import {
  TableContainer,
  Paper,
  Table,
  TableHead,
  TableRow,
  TableCell,
  TableBody,
  TableContainerProps,
  TablePagination,
  Box,
} from '@mui/material';
import TablePaginationActions from '@mui/material/TablePagination/TablePaginationActions';
import { TableInstance } from 'react-table';
import { CustomColumn } from './useCustomTable';
import { PageWithTotal } from './usePage';

export interface CustomTableProps<D extends object> extends Omit<TableContainerProps, 'ref'> {
  tableInstance: TableInstance<D>;
  page?: PageWithTotal;
  containerProps?: TableContainerProps;
}

export function CustomTable<D extends object>({
  tableInstance,
  page,
  containerProps,
  ...tableProps
}: CustomTableProps<D>): JSX.Element {
  const { getTableProps, getTableBodyProps, headerGroups, rows, prepareRow } = tableInstance;
  return (
    <TableContainer
      sx={{
        overflow: 'hidden',
        flex: '1 1 0',
        display: 'flex',
        flexDirection: 'column',
        maxHeight: '100%',
      }}
      component={Paper}
      {...containerProps}
    >
      <Box sx={{ flex: '1 1 0', overflow: 'auto', maxHeight: '100%' }}>
        <Table stickyHeader {...tableProps} {...getTableProps()}>
          <TableHead>
            {
              // Loop over the header rows
              headerGroups.map((headerGroup) => (
                // Apply the header row props
                <TableRow {...headerGroup.getHeaderGroupProps()} key={headerGroup.getHeaderGroupProps().key}>
                  {
                    // Loop over the headers in each row
                    headerGroup.headers.map((column) => {
                      const headerCulumn = column as CustomColumn<D>;
                      const headerProps = headerCulumn.headerCellProps ?? headerCulumn.cellProps ?? {};
                      headerProps.sx = {
                        backgroundColor: (theme) => (theme.palette.mode === 'dark' ? '#2c2c2c' : '#ffffff'),
                        ...headerProps?.sx,
                      };
                      return (
                        // Apply the header cell props
                        <TableCell {...column.getHeaderProps()} {...headerProps} key={column.getHeaderProps().key}>
                          {
                            // Render the header
                            column.render('Header')
                          }
                        </TableCell>
                      );
                    })
                  }
                </TableRow>
              ))
            }
          </TableHead>
          {/* Apply the table body props */}
          <TableBody sx={{ flex: '1 1 0' }} {...getTableBodyProps()}>
            {
              // Loop over the table rows
              rows.map((row) => {
                // Prepare the row for display
                prepareRow(row);
                return (
                  // Apply the row props
                  <TableRow {...row.getRowProps()} key={row.getRowProps().key}>
                    {
                      // Loop over the rows cells
                      row.cells.map((cell) => {
                        const column = cell.column as CustomColumn<D>;

                        // Apply the cell props
                        return (
                          <TableCell {...cell.getCellProps()} {...column.cellProps} key={cell.getCellProps().key}>
                            {
                              // Render the cell contents
                              cell.render('Cell')
                            }
                          </TableCell>
                        );
                      })
                    }
                  </TableRow>
                );
              })
            }
          </TableBody>
        </Table>
      </Box>
      {page && page.total > page.pageSize && (
        <TablePagination
          count={page.total}
          rowsPerPage={page.pageSize}
          page={page.pageIndex}
          onPageChange={(_, p) => {
            page.setPage(p);
          }}
          ActionsComponent={TablePaginationActions}
          rowsPerPageOptions={page.pageSizeOptions}
          onRowsPerPageChange={(event) => {
            page.setPageSize(parseInt(event.target.value, 10));
            page.setPage(0);
          }}
        />
      )}
    </TableContainer>
  );
}

export * from './usePage';

export * from './TableActions';
export * from './useCustomTable';
