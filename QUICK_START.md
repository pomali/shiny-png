# Quick Start Guide

## Installation

```bash
npm install shiny-png
```

Or use the local build from the `pkg/` directory.

## Browser Usage

### 1. Basic Setup

```html
<script type="module">
  import * as shinyPng from './pkg/shiny_png.js';
  
  // Initialize the module
  const init = async () => {
    // WASM is ready to use
  };
  
  init();
</script>
```

### 2. Load and Process Images

```javascript
// Read file from input
const sourceFile = document.getElementById('sourceInput').files[0];
const targetFile = document.getElementById('targetInput').files[0];

// Convert to ArrayBuffer
const [sourceBuf, targetBuf] = await Promise.all([
  sourceFile.arrayBuffer(),
  targetFile.arrayBuffer()
]);

// Convert to Uint8Array
const sourceData = new Uint8Array(sourceBuf);
const targetData = new Uint8Array(targetBuf);

// Transfer ICC profile
const result = shinyPng.transfer_icc_profile(sourceData, targetData);

// Save to file
const blob = new Blob([result], { type: 'image/png' });
const url = URL.createObjectURL(blob);
const a = document.createElement('a');
a.href = url;
a.download = 'result.png';
a.click();
```

## Node.js Usage

### 1. Installation

```bash
npm install shiny-png
```

### 2. Simple Example

```javascript
import * as shinyPng from 'shiny-png';
import fs from 'fs';

const sourceData = fs.readFileSync('source.png');
const targetData = fs.readFileSync('target.png');

const result = shinyPng.transfer_icc_profile(sourceData, targetData);
fs.writeFileSync('result.png', result);
```

### 3. Extract Profile

```javascript
const profile = shinyPng.extract_icc_profile(sourceData);
console.log(`Profile size: ${profile.length} bytes`);
```

### 4. Apply Specific Profile

```javascript
const newImage = shinyPng.apply_icc_profile(targetData, profile);
fs.writeFileSync('new-with-profile.png', newImage);
```

## Error Handling

All functions throw JavaScript errors. Use try-catch:

```javascript
try {
  const result = shinyPng.transfer_icc_profile(source, target);
} catch (error) {
  console.error('Failed to process image:', error);
}
```

## Common Patterns

### Batch Processing

```javascript
import * as shinyPng from 'shiny-png';
import fs from 'fs';
import path from 'path';

async function applyProfileToAll(sourceFile, inputDir, outputDir) {
  const sourceData = fs.readFileSync(sourceFile);
  const profile = shinyPng.extract_icc_profile(sourceData);
  
  const files = fs.readdirSync(inputDir);
  
  for (const file of files) {
    if (!file.endsWith('.png')) continue;
    
    const inputPath = path.join(inputDir, file);
    const outputPath = path.join(outputDir, `profiled-${file}`);
    
    const imageData = fs.readFileSync(inputPath);
    const result = shinyPng.apply_icc_profile(imageData, profile);
    
    fs.writeFileSync(outputPath, result);
    console.log(`✓ Processed: ${file}`);
  }
}

applyProfileToAll('template.png', './images', './output');
```

### Verify Profile

```javascript
function verifyProfile(imageFile) {
  const data = fs.readFileSync(imageFile);
  const profile = shinyPng.extract_icc_profile(data);
  
  if (profile.length === 0) {
    console.log('⚠ No ICC profile found');
    return false;
  }
  
  console.log(`✓ ICC profile found: ${profile.length} bytes`);
  return true;
}

verifyProfile('image.png');
```

## Performance Tips

1. **Process Multiple Images**: Extract the profile once, apply to many images
2. **Stream Large Files**: For very large images, consider streaming decompression
3. **Batch Operations**: Process multiple images in a single JavaScript execution

## Debugging

### Check WASM Loading

```javascript
import init from './pkg/shiny_png.js';

init().then(() => {
  console.log('✓ WASM module loaded');
}).catch(err => {
  console.error('✗ WASM loading failed:', err);
});
```

### Validate PNG Files

```javascript
function isPngFile(data) {
  // PNG files start with 0x89 0x50 0x4E 0x47
  const signature = [0x89, 0x50, 0x4E, 0x47];
  return data.length >= 4 && 
    signature.every((byte, i) => data[i] === byte);
}

const data = new Uint8Array(buffer);
if (!isPngFile(data)) {
  console.error('Invalid PNG file');
}
```

## Limitations & Considerations

- **Input must be valid PNG**: Non-PNG data will throw an error
- **ICC Profile Required**: `transfer_icc_profile` requires source to have a profile
- **Memory**: Entire image is loaded into memory for processing
- **Size**: WASM binary is ~253 KB (compressed: ~80 KB with gzip)

## API Cheat Sheet

```javascript
// Extract ICC profile from PNG
const profile = shinyPng.extract_icc_profile(imageData);

// Apply ICC profile to PNG
const profiled = shinyPng.apply_icc_profile(imageData, profile);

// Transfer ICC profile between images
const result = shinyPng.transfer_icc_profile(sourceImage, targetImage);

// Apply the default embedded ICC profile
const withDefault = shinyPng.apply_default_icc_profile(imageData);

// Get the embedded default ICC profile
const defaultProfile = shinyPng.get_default_icc_profile();
```

## Real-World Example: Web App

```html
<!DOCTYPE html>
<html>
<body>
  <input type="file" id="source" accept="image/png">
  <input type="file" id="target" accept="image/png">
  <button id="process">Process</button>
  
  <script type="module">
    import * as shinyPng from './pkg/shiny_png.js';
    
    const sourceInput = document.getElementById('source');
    const targetInput = document.getElementById('target');
    const processBtn = document.getElementById('process');
    
    processBtn.addEventListener('click', async () => {
      const source = new Uint8Array(await sourceInput.files[0].arrayBuffer());
      const target = new Uint8Array(await targetInput.files[0].arrayBuffer());
      
      try {
        const result = shinyPng.transfer_icc_profile(source, target);
        
        const blob = new Blob([result], { type: 'image/png' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'result.png';
        a.click();
      } catch (error) {
        alert('Error: ' + error);
      }
    });
  </script>
</body>
</html>
```

## Support & Issues

For issues or questions:
1. Check that input files are valid PNG images
2. Verify WASM binary is loaded correctly
3. Check browser console for detailed error messages
4. Ensure sufficient memory is available

