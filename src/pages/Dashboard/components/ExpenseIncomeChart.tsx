import { AreaChart } from '@mantine/charts';

export default function ExpenseIncomeChart() {
  const data = [
    { date: '2023.07.01', Expense: 1200, Income: 400 },
    { date: '2023.08.01', Expense: 1200, Income: 400 },
    { date: '2023.09.01', Expense: 1200, Income: 400 },
    { date: '2023.10.01', Expense: 1200, Income: 400 },
    { date: '2023.11.01', Expense: 1200, Income: 400 },
    { date: '2023.12.01', Expense: 1200, Income: 400 },
    { date: '2024.01.01', Expense: 1200, Income: 400 },
    { date: '2024.02.01', Expense: 1500, Income: 700 },
    { date: '2024.03.01', Expense: 1100, Income: 300 },
    { date: '2024.04.01', Expense: 1300, Income: 600 },
    { date: '2024.05.01', Expense: 1250, Income: 500 },
    { date: '2024.06.01', Expense: 1600, Income: 450 },
    { date: '2024.07.01', Expense: 1400, Income: 350 },
    { date: '2024.08.01', Expense: 1700, Income: 750 },
    { date: '2024.09.01', Expense: 1234, Income: 125 },
  ];

  return (
    <AreaChart
      h={300}
      data={data}
      dataKey="date"
      series={[
        { name: 'Expense', color: 'indigo.6' },
        { name: 'Income', color: 'blue.6' },
      ]}
      curveType="natural"
      tickLine="none"
      gridAxis="none"
      withDots={false}
    />
  );
}