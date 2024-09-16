import { Group, AppShell, ScrollArea, Text } from '@mantine/core';
import {
  IconDashboard,
  IconCalendarDollar,
  IconPigMoney,
  IconSettings,
  IconDeviceIpadHorizontal,
  IconSwitchHorizontal,
  IconLogout,
  IconBusinessplan,
} from '@tabler/icons-react';
import classes from './Navbar.module.css';

type Page = 'Dashboard' | 'Account' | 'Transactions' | 'Budget' | 'Settings';

const data = [
  { label: 'Dashboard', icon: IconDashboard },
  { label: 'Account', icon: IconDeviceIpadHorizontal },
  { label: 'Transactions', icon: IconCalendarDollar },
  { label: 'Budget', icon: IconPigMoney },
  { label: 'Settings', icon: IconSettings },
];

interface NavbarProps {
  activePage: Page;
  setActivePage: (page: Page) => void;
}

export default function Navbar({ activePage, setActivePage }: NavbarProps) {
  const links = data.map((item) => (
    <a
      key={item.label}
      className={classes.link}
      data-active={item.label === activePage || undefined}
      onClick={(event) => {
        event.preventDefault();
        setActivePage(item.label as Page); // Update the active page
      }}
    >
      <item.icon className={classes.linkIcon} stroke={1.5} />
      <Text className={classes.linkText}>{item.label}</Text>
    </a>
  ));

  return (
    <AppShell.Navbar className={classes.navbar} p={{ sm: "sm" }}>
      <AppShell.Section>
        <Group className={classes.header} justify="space-between">
          <IconBusinessplan className={classes.linkIcon} stroke={1.5} />
          <Text className={classes.linkText} size="xl">Finance</Text>
        </Group>
      </AppShell.Section>
      <AppShell.Section className={classes.navbarMain} grow component={ScrollArea}>
        {links}
      </AppShell.Section>

      <AppShell.Section className={classes.footer}>
        <a href="#" className={classes.link} onClick={(event) => event.preventDefault()}>
          <IconSwitchHorizontal className={classes.linkIcon} stroke={1.5} />
          <Text className={classes.linkText}>Change account</Text>
        </a>

        <a href="#" className={classes.link} onClick={(event) => event.preventDefault()}>
          <IconLogout className={classes.linkIcon} stroke={1.5} />
          <Text className={classes.linkText}>Logout</Text>
        </a>
      </AppShell.Section>
    </AppShell.Navbar>
  );
}