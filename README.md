# MD Collate

MDCollate is a Markdown Collator written in Rust.  

It will take an entry file, resolving all internal file links and collating them into the one markdown file, sprinkling it with anchors so internal links work.  The output is currently to stdout to pipe to a tool such as `cmark` or `pulldown-cmark`

## Building

This project uses Cargo

```
cargo build
```

## Example

The `data` Directory of this project currently includes some test markdown files which should resolve correctly.  You can run

```
cargo run data/test.md
```

Or if you have mdcollate on your path:

```
mdcollate data/test.md
```

### Creating a PDF

You can create a PDF by combining a few tools such as `pulldown-cmark` and `wkhtmltopdf`, converting to HTML first:

```
mdcollate data/test.md | pulldown-cmark > test.html && wkhtmltopdf test.html test.pdf
```

## Known caveats

* Markdown doesn't have any shortcut formats for anchors, so we need to write them as HTML
* `wkhtmltopdf` has [issues](https://github.com/wkhtmltopdf/wkhtmltopdf/issues/1554) with anchors that are not visible, so we use `<p>` blocks instead
* Images will only work in the directory where you run the command, so if you move the collated md file out of the dir, then you will need to update the links
