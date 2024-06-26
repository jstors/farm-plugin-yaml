import { createRoot } from 'react-dom/client';
import { Main } from './main';
import './index.css'
import test from './test.yml'


console.log('YML文件', test)

const container = document.querySelector('#root');
const root = createRoot(container||document.body);

root.render(<Main />);
