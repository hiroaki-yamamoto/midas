import { createBrowserRouter } from 'react-router-dom';
import App from './dashboard/view.tsx';
import { BotEditor } from './bot-editor/view.tsx';

export const routing = createBrowserRouter([
  {
    path: '/',
    element: <App />,
  },
  {
    path: '/new-bot',
    element: <BotEditor />,
  }
]);
