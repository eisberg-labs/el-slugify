# node-el-slugify

Fast and efficient URL slug generator written in Rust.
Strings are sanitized and transliterated.

## Installation
```yaml
npm i --save node-el-slugify
```

## Installation in repository

Installing nodejs bindings require a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

This fully installs the project, including installing any dependencies and running the build.

## Building nodejs bindings

If you have already installed the project and only want to run the build, run:

```sh
npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./build/Release/index.node`.
Prebuild requires that the binary is in `build/Release` as though it was built with node-gyp.

## Example
```javascript
const slugifier = require("node-el-slugify");

assert.strictEqual(slugifier.slugify('mačka Mački Grize rep!'), 'macka-macki-grize-rep')
assert.strictEqual(slugifier.slugify_with_replacement('mačka Mački Grize rep!', '_'), 'macka_macki_grize_rep')
```

## Troubleshooting

> Module parse failed: Unexpected character. You may need an appropriate loader to handle this file type, currently no loaders are configured to process this file.

That error might happen if you're using it in NextJS, or any webpack related project.
You can fix it by including a `node-loader`. Your `next.config.js` might look like:
```javascript
module.exports = {
    webpack: (config, {dev, isServer, webpack, nextRuntime}) => {
        config.module.rules = [
            ...config.module.rules,
            {
                test: /\.node$/,
                loader: "node-loader",
            },

        ];

        return config;
    }
}
```
