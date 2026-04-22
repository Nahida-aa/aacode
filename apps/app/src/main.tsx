/* @refresh reload */

import { RouterProvider } from '@tanstack/solid-router';
import { render } from 'solid-js/web';
import './styles.css';
import { getRouter } from '#/router.tsx';

// Create a new router instance
const router = getRouter();

// Render the app
const rootElement = document.getElementById('root')!;

render(() => <RouterProvider router={router} />, rootElement);
