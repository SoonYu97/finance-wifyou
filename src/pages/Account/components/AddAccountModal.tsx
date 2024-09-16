import { ActionIcon, Tooltip, TextInput, NumberInput, Button, Flex } from "@mantine/core";
import { useForm } from '@mantine/form';
import { invoke } from '@tauri-apps/api/core';
import { modals } from '@mantine/modals';
import { notifications } from '@mantine/notifications';
import { IconDeviceIpadHorizontalPlus } from "@tabler/icons-react"

interface Account {
  name: string;
  account_type: string;
  balance: number;
  currency: string;
  note?: string;
}

export default function AddAccountModal() {
  const form = useForm({
    mode: 'uncontrolled',
    initialValues: { name: '', account_type: '', balance: 0, currency: '', note: '' },
    validate: {
      name: (value) => (value.length < 3 ? 'Name must have at least 3 letters' : null),
      account_type: (value) => (value.length < 1 ? 'Type must have at least 1 letters' : null),
      currency: (value) => (value.length < 1 ? 'Currency must have at least 1 letters' : null),
    },
  });

  const handleError = (errors: typeof form.errors) => {
    if (errors.name) {
      notifications.show({ message: 'Please fill name field', color: 'red' });
    } else if (errors.account_type) {
      notifications.show({ message: 'Please fill type field', color: 'red' });
    } else if (errors.currency) {
      notifications.show({ message: 'Please fill currency field', color: 'red' });
    }
  };

  const addAccount = async (values: Account) => {
    try {
      const result = await invoke('create_account', {
        name: values.name,
        account_type: values.account_type,
        balance: values.balance,
        currency: values.currency,
        note: values.note || null
      });
      console.log(result)
      notifications.show({
        title: 'Account Added',
        message: "Account added successfully",
        color: 'green'
      })
    } catch (error) {
      console.log(error)
      notifications.show({
        title: 'Account Not Added',
        message: "Failed to add account",
      })
    } finally {
      form.reset();
      modals.closeAll();
    }
  }

  const openModal = () => modals.open({
    title: 'Add an Account',
    children: (
      <form onSubmit={form.onSubmit((values: Account) => addAccount(values), handleError)}>
        <Flex
          direction={"column"}
          gap={"lg"}
          justify={"start"}
          align={"stretch"}

        >
          <TextInput
            withAsterisk
            label="Name" placeholder="Name" data-autofocus
            key={form.key('name')}
            {...form.getInputProps('name')}
          />
          <TextInput
            withAsterisk
            label="Type" placeholder="Type"
            key={form.key('account_type')}
            {...form.getInputProps('account_type')}
          />
          <NumberInput
            label="Balance"
            defaultValue={0}
            key={form.key('balance')}
            {...form.getInputProps('balance')}
          />
          <TextInput
            withAsterisk
            label="Currency" placeholder="Currency"
            key={form.key('currency')}
            {...form.getInputProps('currency')}
          />
          <TextInput
            label="Note" placeholder="Note"
            key={form.key('note')}
            {...form.getInputProps('note')}
          />
          <Button fullWidth type='submit' mt="md">
            Add an account
          </Button>
        </Flex>
      </form>
    ),
  });

  return (
    <Tooltip label="Click to add an account">
      <ActionIcon onClick={openModal} size="lg" variant="filled" aria-label="Add a transaction">
        <IconDeviceIpadHorizontalPlus style={{ width: '70%', height: '70%' }} stroke={1.5} />
      </ActionIcon >
    </Tooltip>
  )
}