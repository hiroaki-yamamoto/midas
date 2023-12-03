import { useState, MouseEvent } from 'react';

import TableContainer from '@mui/material/TableContainer';
import TableCell from '@mui/material/TableCell';
import TableSortLabel from '@mui/material/TableSortLabel';
import Table from '@mui/material/Table';

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

export function PositionTable(input: { positions: Position }) {
  const [order, setOrder] = useState<Direction>(Direction.Asc);
  const [orderBy, setOrderBy] = useState<TableHeaderLabel>('Symbol');
  const onSortRequest = (
    event: MouseEvent<unknown>,
    property: TableHeaderLabel
  ) => {
    const isAsc = orderBy === property && order === Direction.Asc;
    setOrder(isAsc ? Direction.Desc : Direction.Asc);
    setOrderBy(property);
  };
  return (
    <TableContainer>
      <Table>
        <TableHeader
          onSortRequest={onSortRequest}
          order={order}
          orderBy={orderBy} />
      </Table>
    </TableContainer>
  );
}
