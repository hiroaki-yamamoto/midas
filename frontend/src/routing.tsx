import { createBrowserRouter } from 'react-router-dom';
import { Dashboard } from './dashboard/view.tsx';
import { BotEditor } from './bot-editor/view.tsx';
import { Root } from './root/view.tsx';

export const routing = createBrowserRouter([
  {
    path: '/',
    element: <Root />,
    children: [
      {
        path: '/',
        element: <Dashboard />,
      },
      {
        path: '/new-bot',
        element: <BotEditor />,
      }
    ],
  },
]);
