import {
  TableContainer,
  Paper,
  Table,
  TableHead,
  TableRow,
  TableCell,
  TableBody,
  TableContainerProps,
  TableFooter,
  TablePagination,
} from '@mui/material';
import TablePaginationActions from '@mui/material/TablePagination/TablePaginationActions';
import { TableInstance } from 'react-table';
import { PageState } from './usePage';

export interface CustomTableProps<D extends object> extends TableContainerProps {
  tableInstance: TableInstance<D>;
  page?: Omit<PageState, 'limit' | 'offset'> & { total: number };
}

export function CustomTable<D extends object>({ tableInstance, page }: CustomTableProps<D>): JSX.Element {
  const { getTableProps, getTableBodyProps, headerGroups, rows, prepareRow } = tableInstance;
  return (
    <TableContainer component={Paper}>
      <Table {...getTableProps()}>
        <TableHead>
          {
            // Loop over the header rows
            headerGroups.map((headerGroup, index) => (
              // Apply the header row props
              <TableRow {...headerGroup.getHeaderGroupProps()} key={index}>
                {
                  // Loop over the headers in each row
                  headerGroup.headers.map((column, index) => (
                    // Apply the header cell props
                    <TableCell {...column.getHeaderProps()} key={index}>
                      {
                        // Render the header
                        column.render('Header')
                      }
                    </TableCell>
                  ))
                }
              </TableRow>
            ))
          }
        </TableHead>
        {/* Apply the table body props */}
        <TableBody {...getTableBodyProps()}>
          {
            // Loop over the table rows
            rows.map((row, index) => {
              // Prepare the row for display
              prepareRow(row);
              return (
                // Apply the row props
                <TableRow {...row.getRowProps()} key={index}>
                  {
                    // Loop over the rows cells
                    row.cells.map((cell, index) => {
                      // Apply the cell props
                      return (
                        <TableCell {...cell.getCellProps()} key={index}>
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
        {page && page.total > page.pageSize && (
          <TableFooter>
            <TableRow>
              <TablePagination
                colSpan={3}
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
            </TableRow>
          </TableFooter>
        )}
      </Table>
    </TableContainer>
  );
}

export * from './usePage';

export * from './TableActions';
