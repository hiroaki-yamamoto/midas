import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';
import AppBar from '@mui/material/AppBar';
import Typography from '@mui/material/Typography';

import { routing } from './routing.tsx';
import './index.scss';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AppBar>
      <Typography variant="h5" component="h6">Midas</Typography>
    </AppBar>
    <RouterProvider router={routing} />
  </React.StrictMode >,
);
