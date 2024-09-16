import { useState } from 'react'
import { AppShell } from '@mantine/core';
import Navbar from './components/Navbar/Navbar';

import Account from './pages/Account/Account';
import Budget from './pages/Budget/Budget';
import Dashboard from './pages/Dashboard/Dashboard';
import Settings from './pages/Settings/Settings';
import Transactions from './pages/Transactions/Transactions';
import ActionAffix from './components/ActionAffix/ActionAffix';

type Page = 'Dashboard' | 'Account' | 'Transactions' | 'Budget' | 'Settings';

const pages: Record<Page, JSX.Element> = {
  Dashboard: <Dashboard />,
  Account: <Account />,
  Transactions: <Transactions />,
  Budget: <Budget />,
  Settings: <Settings />,
};

function App() {
  const [activePage, setActivePage] = useState<Page>('Dashboard');

  return (
    <AppShell
      navbar={{
        width: { sm: 200, lg: 300 },
        breakpoint: '0',
      }}
      padding="md"
    >
      <Navbar activePage={activePage} setActivePage={setActivePage} />

      <AppShell.Main ml={{ base: "45px", sm: "0" }}>
        {pages[activePage]}
      </AppShell.Main>
      <ActionAffix />
    </AppShell>
  );
}

export default App;
