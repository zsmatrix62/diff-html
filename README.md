# diff-html

A WebAssembly plugin for Extism that performs HTML diffing between two HTML documents.

For more information about using Extism plugins, see the [Extism official documentation](https://extism.org/docs/overview).

## Features

- Performs semantic HTML diffing
- Preserves document structure
- Highlights changes with <ins> and <del> tags
- Works as a lightweight WebAssembly module

## Example

### Input JSON
```json
{
  "before": "<div><p>Original content with some text</p></div>",
  "after": "<div><p>Modified content with different text</p></div>"
}
```

### Output HTML
```html
<div>
  <p>
    <del>Original</del><ins>Modified</ins> content 
    <del>with some</del><ins>with different</ins> text
  </p>
</div>
```

### Rendered Output
```html
<div>
  <p>
    <del style="color:red; text-decoration:line-through">Original</del>
    <ins style="color:green; text-decoration:underline">Modified</ins> content 
    <del style="color:red; text-decoration:line-through">with some</del>
    <ins style="color:green; text-decoration:underline">with different</ins> text
  </p>
</div>
```

## Installation

1. Install Extism CLI and Rust toolchain:
   ```bash
   make setup
   ```

2. Build the optimized WebAssembly module:
   ```bash
   make build
   ```

3. The compiled WebAssembly module will be available at:
   ```
   target/wasm32-unknown-unknown/release/diff_html.wasm
   ```

## Usage with Extism

### Rust Example

```rust
use extism_pdk::*;

#[plugin_fn]
pub fn diff_html(input: String) -> FnResult<String> {
    let input: serde_json::Value = serde_json::from_str(&input)?;
    
    let before = input["before"].as_str().unwrap();
    let after = input["after"].as_str().unwrap();
    
    let result = diff_html_rs::diff(before, after);
    Ok(result)
}
```

### JavaScript Example

```javascript
import { Plugin } from 'extism';

async function diffHtml(before, after) {
  const plugin = await Plugin.fromWasmFile(
    './target/wasm32-unknown-unknown/release/diff_html.wasm'
  );

  const input = JSON.stringify({
    before,
    after
  });

  const output = await plugin.call('diff_html', input);
  return output.text();
}

// Example usage
const beforeHtml = '<div class="container"><p id="p1">Content</p></div>';
const afterHtml = '<div class="wrapper"><p id="p1" style="color:red">Modified Content</p></div>';

diffHtml(beforeHtml, afterHtml).then(result => {
  console.log('Diff result:', result);
});
```

## License

MIT
