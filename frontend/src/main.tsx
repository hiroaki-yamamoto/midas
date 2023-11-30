import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { indigo } from '@mui/material/colors';

import { routing } from './routing.tsx';
import style from './root.module.scss';
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
      <AppBar position="static" enableColorOnDark>
        <Toolbar>
          <Typography variant="h6" component="h6">Midas</Typography>
        </Toolbar>
      </AppBar>
      <div className={style.container}>
        <RouterProvider router={routing} />
      </div>
    </ThemeProvider>
  </React.StrictMode >,
);
