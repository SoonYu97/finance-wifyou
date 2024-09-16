import { Title, Text } from '@mantine/core';
import '@mantine/charts/styles.css';
import ExpenseIncomeChart from './components/ExpenseIncomeChart';

export default function Dashboard() {
  return (
    <div>
      <Title>Dashboard</Title>
      <Text>Welcome to the Dashboard</Text>
      <ExpenseIncomeChart />
    </div>
  );
}