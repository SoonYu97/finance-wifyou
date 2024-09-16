import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import { createTheme, MantineProvider } from '@mantine/core';
import { ModalsProvider } from '@mantine/modals';
import { Notifications } from '@mantine/notifications';

const theme = createTheme({
  /** Put your mantine theme override here */
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(

  <MantineProvider theme={theme} defaultColorScheme="dark">
    <Notifications />
    <ModalsProvider>
      <React.StrictMode>
        <App />
      </React.StrictMode>
      </ModalsProvider>
  </MantineProvider>
);
