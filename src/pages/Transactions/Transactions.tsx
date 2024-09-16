import { Title, Text, Table, TableData } from '@mantine/core';

export default function Transactions() {
  const tableData: TableData = {
    head: ['Time', 'Category', 'Note', 'Amount'],
    body: [
      [6, 12.011, 'C', 'Carbon'],
      [7, 14.007, 'N', 'Nitrogen'],
      [39, 88.906, 'Y', 'Yttrium'],
      [56, 137.33, 'Ba', 'Barium'],
      [58, 140.12, 'Ce', 'Cerium'],
    ],
  };

  return (
    <div>
      <Title>Transactions</Title>
      <Text>Welcome to the Transactions</Text>
      <Table.ScrollContainer minWidth={500} type="native">
        <Table highlightOnHover data={tableData} />
      </Table.ScrollContainer>
    </div>
  );
}