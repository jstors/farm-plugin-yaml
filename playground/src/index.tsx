import React from 'react';
import { createRoot } from 'react-dom/client';
import { Main } from './main';
import './index.css'
import test from './test.yaml'
console.log('tttt=》', test,test["2fa"])

const container = document.querySelector('#root');
const root = createRoot(container);

root.render(<Main />);
