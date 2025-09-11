import '@/main.css';
import router from '@/router/index';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux';
import { RouterProvider } from 'react-router';
import { store } from './store';

const root = document.getElementById('root');

if (root) {
  createRoot(root).render(
    <StrictMode>
      <Provider store={store}>
        <RouterProvider router={router} unstable_onError={(error, errorInfo) => {
          console.error(error, errorInfo);
        }} />
      </Provider>
    </StrictMode>,
  )
}