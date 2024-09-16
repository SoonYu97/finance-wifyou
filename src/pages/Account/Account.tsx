import { Flex } from '@mantine/core';
import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import AddAccountModal from './components/AddAccountModal';

interface Account {
  id: number;
  name: string;
  account_type: string;
  balance: number;
  currency: string;
  note?: string;
}

export default function Account() {
  const [accounts, setAccounts] = useState<Account[]>([]);

  useEffect(() => {
    async function fetchAccounts() {
      try {
        const result = await invoke<Account[]>('get_accounts');
        setAccounts(result);
      } catch (error) {
        console.error('Failed to fetch accounts:', error);
      }
    }
    fetchAccounts();
  }, [accounts]);

  return (
    <div>
      <Flex gap={'md'} align={'center'}>
        <h1>Accounts</h1>
        <AddAccountModal />
      </Flex>
      <ul>
        {accounts.map(account => (
          <li key={account.id}>
            {account.name} ({account.currency}) - {account.balance}
          </li>
        ))}
      </ul>
    </div>
  );
}
