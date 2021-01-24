const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const appConfig = {
  entry: "./main.js",
  devServer: {
    contentBase: dist,
    port: 8080,
    host: "0.0.0.0"
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "index.html"
    })
  ],
  resolve: {
    extensions: [".js"]
  },
  output: {
    path: dist,
    filename: "app.js"
  }
};

const workerConfig = {
  entry: "./worker.js",
  target: "webworker",
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "./"),
      forceMode: 'production',
      watchDirectories: ['../engine']
    })
  ],
  resolve: {
    extensions: [".js", ".wasm"]
  },
  output: {
    path: dist,
    filename: "worker.js"
  }
};

module.exports = [appConfig, workerConfig];
