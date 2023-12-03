import { useState, MouseEvent, useMemo, ChangeEvent } from 'react';

import Box from '@mui/material/Box';
import TableContainer from '@mui/material/TableContainer';
import TableCell from '@mui/material/TableCell';
import TableSortLabel from '@mui/material/TableSortLabel';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableRow from '@mui/material/TableRow';
import TablePagination from '@mui/material/TablePagination';

import { Position } from '../rpc/position.zod';

const TableHeaderLabel = [
  'Symbol', 'Trading Amount', 'Valuation', 'Profit Amount', 'Profit %'
];
type TableHeaderLabel =
  'Symbol' | 'Trading Amount' | 'Valuation' | 'Profit Amount' | 'Profit %';

enum Direction {
  Asc = 'asc',
  Desc = 'desc',
}

const TableHeader = (input: {
  orderBy: TableHeaderLabel,
  order: Direction,
  onSortRequest: (
    event: MouseEvent<unknown>, property: TableHeaderLabel
  ) => void,
}) => {
  const sortHandler = (prop: TableHeaderLabel) => {
    return (event: MouseEvent<unknown>) => {
      input.onSortRequest(event, prop);
    };
  };
  return TableHeaderLabel.map((header) => {
    const direction = input.orderBy === header ? input.order : 'asc';
    return (
      <TableCell key={header}
        sortDirection={direction}>
        <TableSortLabel
          active={input.orderBy === header}
          direction={direction}
          onClick={sortHandler(header as TableHeaderLabel)}>
          {header}
        </TableSortLabel>
      </TableCell>
    );
  });
};

export function PositionTable(input: { positions: Position[] }) {
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  const [order, setOrder] = useState<Direction>(Direction.Asc);
  const [orderBy, setOrderBy] = useState<TableHeaderLabel>('Symbol');
  const onSortRequest = (
    _event: MouseEvent<unknown>,
    property: TableHeaderLabel
  ) => {
    const isAsc = orderBy === property && order === Direction.Asc;
    setOrder(isAsc ? Direction.Desc : Direction.Asc);
    setOrderBy(property);
  };
  const onChangePage = (_event: unknown, newPage: number) => {
    setPage(newPage);
  };
  const onChangeRowsPerPage = (event: ChangeEvent<HTMLInputElement>) => {
    setRowsPerPage(parseInt(event.target.value, 10));
    setPage(0);
  };

  const positions = useMemo(() => {
    return input.positions.sort((a, b) => {
      const isAsc = order === Direction.Asc;
      switch (orderBy) {
        case 'Symbol':
          return isAsc
            ? a.symbol.localeCompare(b.symbol)
            : b.symbol.localeCompare(a.symbol);
        case 'Trading Amount':
          return isAsc
            ? parseFloat(a.trading_amount) - parseFloat(b.trading_amount)
            : parseFloat(b.trading_amount) - parseFloat(a.trading_amount);
        case 'Valuation':
          return isAsc
            ? parseFloat(a.valuation) - parseFloat(b.valuation)
            : parseFloat(b.valuation) - parseFloat(a.valuation);
        case 'Profit Amount':
          return isAsc
            ? parseFloat(a.profit_amount) - parseFloat(b.profit_amount)
            : parseFloat(b.profit_amount) - parseFloat(a.profit_amount);
        case 'Profit %':
          return isAsc
            ? parseFloat(a.profit_percent) - parseFloat(b.profit_percent)
            : parseFloat(b.profit_percent) - parseFloat(a.profit_percent);
        default:
          return 0;
      }
    }).slice(
      page * rowsPerPage, page * rowsPerPage * 2
    );
  }, [input, order, orderBy, page, rowsPerPage]);

  return (
    <Box>
      <TableContainer>
        <Table>
          <TableHeader
            onSortRequest={onSortRequest}
            order={order}
            orderBy={orderBy} />
          <TableBody>
            {
              positions.map((pos) => {
                return (
                  <TableRow key={pos.id} hover>
                    <TableCell>{pos.symbol}</TableCell>
                    <TableCell>{pos.trading_amount}</TableCell>
                    <TableCell>{pos.valuation}</TableCell>
                    <TableCell>{pos.profit_amount}</TableCell>
                    <TableCell>{pos.profit_percent}</TableCell>
                  </TableRow>
                );
              })
            }
          </TableBody>
        </Table>
      </TableContainer>
      <TablePagination
        rowsPerPageOptions={[5, 10, 25, 50]}
        component='div' count={positions.length}
        rowsPerPage={rowsPerPage} page={page}
        onPageChange={onChangePage}
        onRowsPerPageChange={onChangeRowsPerPage} />
    </Box>
  );
}
