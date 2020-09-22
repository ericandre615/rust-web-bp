const path = require('path');

module.exports = {
  entry: './static/main.js',
  //output: {
  //  filename: 'main.js',
  //  path: path.resolve(__dirname, 'public'),
  //  publicPath: '/',
  //},
  //devtool: 'inline-source-map',
  devServer: {
    contentBase: './static',
    historyApiFallback: true,
  },
};

