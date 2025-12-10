# Shiny PNG - WebAssembly Library

A WebAssembly library for manipulating ICC color profiles in PNG images.

## Features

- **Apply ICC Profile**: Add an ICC profile to any PNG image
- **Extract ICC Profile**: Get the ICC profile from a PNG image
- **Transfer ICC Profile**: Copy the ICC profile from one image to another

## Installation

```bash
npm install shiny-png
```

## Usage

### Node.js

```javascript
import * as shinyPng from 'shiny-png';
import fs from 'fs';

// Read PNG files
const sourceImage = fs.readFileSync('source.png');
const targetImage = fs.readFileSync('target.png');

// Extract ICC profile from source
const profile = shinyPng.extract_icc_profile(sourceImage);
console.log(`Extracted profile size: ${profile.length} bytes`);

// Apply profile to target image
const result = shinyPng.apply_icc_profile(targetImage, profile);
fs.writeFileSync('result.png', result);

// Or transfer directly
const transferred = shinyPng.transfer_icc_profile(sourceImage, targetImage);
fs.writeFileSync('transferred.png', transferred);
```

### Browser

```html
<script type="module">
  import * as shinyPng from './pkg/shiny_png.js';

  // Assuming you have File inputs for images
  const sourceFile = document.getElementById('sourceImage').files[0];
  const targetFile = document.getElementById('targetImage').files[0];

  Promise.all([
    sourceFile.arrayBuffer(),
    targetFile.arrayBuffer()
  ]).then(([sourceBuf, targetBuf]) => {
    const sourceData = new Uint8Array(sourceBuf);
    const targetData = new Uint8Array(targetBuf);

    // Transfer ICC profile from source to target
    const result = shinyPng.transfer_icc_profile(sourceData, targetData);
    
    // Create download link
    const blob = new Blob([result], { type: 'image/png' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'result.png';
    a.click();
  });
</script>
```

## API

### `apply_icc_profile(imageData: Uint8Array, iccProfile: Uint8Array): Uint8Array`

Applies an ICC color profile to a PNG image.

**Parameters:**
- `imageData`: The PNG image data as bytes
- `iccProfile`: The ICC profile data as bytes

**Returns:** The PNG image with the ICC profile embedded

**Throws:** An error if the input is not a valid PNG or if encoding fails

### `extract_icc_profile(imageData: Uint8Array): Uint8Array`

Extracts the ICC profile from a PNG image.

**Parameters:**
- `imageData`: The PNG image data as bytes

**Returns:** The ICC profile data, or an empty array if no profile exists

**Throws:** An error if the input is not a valid PNG

### `transfer_icc_profile(sourceImage: Uint8Array, targetImage: Uint8Array): Uint8Array`

Transfers the ICC profile from one PNG image to another.

**Parameters:**
- `sourceImage`: The PNG image to extract the ICC profile from
- `targetImage`: The PNG image to apply the ICC profile to

**Returns:** The target image with the source image's ICC profile

**Throws:** An error if either input is not a valid PNG, or if the source has no ICC profile

### `apply_default_icc_profile(imageData: Uint8Array): Uint8Array`

Applies the default embedded ICC profile to a PNG image.

The library includes a pre-defined ICC profile that can be applied to any PNG image without needing to provide an external profile.

**Parameters:**
- `imageData`: The PNG image data as bytes

**Returns:** The PNG image with the default ICC profile embedded

**Throws:** An error if the input is not a valid PNG

### `get_default_icc_profile(): Uint8Array`

Returns the embedded default ICC profile.

**Returns:** The default ICC profile data as bytes

This is useful if you want to inspect the profile or apply it to multiple images efficiently.

## Building

To build the WebAssembly library yourself:

```bash
# Install Rust and add the wasm target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen
cargo install wasm-bindgen-cli

# Build the library
npm run build
```