import { Outlet, Link } from 'react-router-dom';

import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';

import style from './style.module.scss';

export const Root = () => {
  return (
    <>
      <AppBar position="static" enableColorOnDark>
        <Toolbar>
          <Typography variant="h6" component="h6">
            <Link to='/'>Midas</Link>
          </Typography>
        </Toolbar>
      </AppBar>
      <div className={style.container}>
        <Outlet />
      </div>
    </>
  );
};
