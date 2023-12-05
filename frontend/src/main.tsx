import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { ThemeProvider, createTheme } from '@mui/material/styles';
import { indigo } from '@mui/material/colors';

import { routing } from './routing.tsx';
import './index.scss';

const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: indigo,
  },
});

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <RouterProvider router={routing} />
    </ThemeProvider>
  </React.StrictMode >,
);
