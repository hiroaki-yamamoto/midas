import { createBrowserRouter } from 'react-router-dom';
import App from './App.tsx';

export const routing = createBrowserRouter([
  {
    path: '/',
    element: <App />,
  }
]);
