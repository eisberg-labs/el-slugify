"use strict";

const assert = require("assert");

const slugifier = require("..");


describe("Slugify", () => {
  it('slugifies with default replacement', () => {
    assert.strictEqual(slugifier.slugify('mačka Mački Grize rep!'), 'macka-macki-grize-rep')
  })

  it('slugifies with custom replacement', () => {
    assert.strictEqual(slugifier.slugify_with_replacement('mačka Mački Grize rep!', '_'), 'macka_macki_grize_rep')
  })
});
