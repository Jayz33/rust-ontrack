const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './src/bootstrap.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
  },  
  devServer: {
    contentBase: './dist'
  },
  plugins: [
      new CopyWebpackPlugin([
          "index.html",
          { from: 'wasm'}
      ])
  ]
};