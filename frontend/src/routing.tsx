import { createBrowserRouter } from 'react-router-dom';
import App from './dashboard.tsx';

export const routing = createBrowserRouter([
  {
    path: '/',
    element: <App />,
  }
]);
