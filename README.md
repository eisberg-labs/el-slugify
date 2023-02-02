# EL-SLUGIFY

> URL slug generator utility. Slugs are generated efficiently, fast, they are transliterated and sanitized for use in URLs.

What makes a good slug?

- transliterated
- lowercased
- sanitized

I wrote a [blog post about it](https://www.amarjanica.com/fast-and-efficient-slugify-written-for-rust-and-node) if you're curious.

You can use el_slugify in your rust and node projects.

## Example in Rust

Dependencies:

```toml
[dependencies]
el-slugify = "0.1.0"
```
And use:
```rust

use el_slugify::{slugify, slugify_with_replacement};

assert_eq!(slugify("#% MaČKA mački grize rep! (RIB-a) ~*"), "macka-macki-grize-rep-rib-a");
assert_eq!(slugify_with_replacement("#% MaČKA mački grize rep! (RIB-a) ~*", '_'), "macka_macki_grize_rep_rib_a");

```

## Example in Node

Node module reuses functionality from el_slugify crate.

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

## License
[MIT](./LICENSE-MIT) © [Eisberg Labs](http://www.eisberg-labs.com)

