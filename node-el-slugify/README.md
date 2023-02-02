# node-el-slugify

Fast and efficient URL slug generator written in Rust.
Strings are sanitized and transliterated.

Dependencies:

```yaml
npm i --save node-el-slugify
```

And use:
```javascript
const slugifier = require("node-el-slugify");

assert.strictEqual(slugifier.slugify('mačka Mački Grize rep!'), 'macka-macki-grize-rep')
assert.strictEqual(slugifier.slugify_with_replacement('mačka Mački Grize rep!', '_'), 'macka_macki_grize_rep')
```
