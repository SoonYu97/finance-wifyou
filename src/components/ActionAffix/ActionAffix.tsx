import { Affix, ActionIcon, Tooltip, TextInput, NumberInput, Button } from "@mantine/core";
import { modals } from '@mantine/modals';
import { notifications } from '@mantine/notifications';
import { IconPlus } from "@tabler/icons-react"

export default function ActionAffix() {
  const openModal = () => modals.open({
    title: 'Add a Transaction',
    children: (
      <>
        <TextInput label="Note" placeholder="Note" data-autofocus />
        <NumberInput label="Amount" />
        <Button fullWidth onClick={() => {
          modals.closeAll();
          notifications.show({
            title: 'Transaction Added',
            message: '???',
          })
        }} mt="md">
          Submit
        </Button>
      </>
    ),
  });

  return (
    <Affix position={{ bottom: 20, right: 20 }}>
      <Tooltip label="Click to add a transaction">
        <ActionIcon onClick={openModal} size="xl" variant="filled" aria-label="Add a transaction">
          <IconPlus style={{ width: '70%', height: '70%' }} stroke={1.5} />
        </ActionIcon >
      </Tooltip>
    </Affix>
  )
}