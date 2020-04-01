import base from './rollup.config.base'
import {resolve} from 'path'
import pkg from "../package.json";

export default {
  ...base,
  output: {
    file: resolve(__dirname, '../', pkg.browser),
    name: 'TinyCompiler',
    format: 'umd',
    sourcemap: true,
  },
  // plugins:[]
}

