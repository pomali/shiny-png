# 🎨 Shiny PNG - WebAssembly Library

## 🚀 Quick Start

```javascript
import * as shinyPng from './pkg/shiny_png.js';

// Transfer ICC profile from one image to another
const result = shinyPng.transfer_icc_profile(sourceImage, targetImage);
```

## 📁 Project Structure

```
shiny-png/
├── src/lib.rs                  # WebAssembly library source
├── examples/
│   ├── analyze.rs              # PNG metadata analyzer
│   └── create.rs               # PNG creator with ICC profile
├── pkg/                        # WebAssembly build output
│   ├── shiny_png_bg.wasm       # 253 KB compiled binary
│   ├── shiny_png.js            # JavaScript wrapper
│   ├── shiny_png.d.ts          # TypeScript definitions
│   └── package.json            # NPM package config
├── index.html                   # Interactive browser demo
├── example.js                  # Node.js example
└── [Documentation files]
```

## 📚 Documentation

- **[COMPLETE_SUMMARY.md](COMPLETE_SUMMARY.md)** - Full technical overview
- **[QUICK_START.md](QUICK_START.md)** - Usage guide with examples
- **[WASM_README.md](WASM_README.md)** - Technical details & API reference
- **[pkg/README.md](pkg/README.md)** - Package documentation

## ✨ Features

✅ Apply ICC profiles to PNG images  
✅ Extract ICC profiles from PNG images  
✅ Transfer ICC profiles between images  
✅ Works in browser and Node.js  
✅ Full TypeScript support  
✅ Optimized for performance & size  

## 🎯 Examples

### Browser
```html
<script type="module">
  import * as shinyPng from './pkg/shiny_png.js';
  
  const result = shinyPng.transfer_icc_profile(sourceData, targetData);
  // Save result as PNG
</script>
```

### Node.js
```bash
node example.js source.png target.png result.png
```

### NPM
```bash
npm install shiny-png
import * as shinyPng from 'shiny-png';
```

## 🔧 Build

```bash
# Install prerequisites
cargo install wasm-bindgen-cli
rustup target add wasm32-unknown-unknown

# Build library
npm run build
# or: cargo build --lib --target wasm32-unknown-unknown --release && wasm-bindgen ...
```

## 📊 Stats

- **Binary Size**: 253 KB (80 KB gzip)
- **Supported Platforms**: Browser, Node.js, any JS runtime with WASM
- **Language**: Rust → WebAssembly
- **API**: JavaScript/TypeScript

## 🎨 Interactive Demo

Open `index.html` in your browser for a full-featured GUI:
- Upload PNG files
- Transfer ICC profiles with visual feedback
- Download results

## 💡 Common Tasks

**Extract profile from image:**
```javascript
const profile = shinyPng.extract_icc_profile(imageData);
```

**Apply profile to image:**
```javascript
const profiled = shinyPng.apply_icc_profile(imageData, profile);
```

**Transfer profile:**
```javascript
const result = shinyPng.transfer_icc_profile(sourceImage, targetImage);
```

## 🧪 Examples

### Rust CLI
```bash
cargo run --example analyze -- image.png
cargo run --example create
```

### Node.js
```bash
node example.js source.png target.png output.png
```

## 📋 API Reference

### `apply_icc_profile(imageData: Uint8Array, iccProfile: Uint8Array): Uint8Array`
Apply an ICC profile to a PNG image.

### `extract_icc_profile(imageData: Uint8Array): Uint8Array`
Extract ICC profile from a PNG image.

### `transfer_icc_profile(sourceImage: Uint8Array, targetImage: Uint8Array): Uint8Array`
Transfer ICC profile from one image to another.

## 🚢 Deployment

### Publish to NPM
```bash
cd pkg && npm publish
```

### Use in Web Project
```bash
npm install shiny-png
# or copy pkg/ directory to your project
```

### Deploy as Service
- Copy `pkg/` to web server
- Serve via HTTP/HTTPS
- Use in JavaScript applications

## 📝 Files Created

| File | Purpose | Lines |
|------|---------|-------|
| `src/lib.rs` | WebAssembly library | 65+ |
| `examples/analyze.rs` | PNG analyzer | 90+ |
| `examples/create.rs` | PNG creator | 80+ |
| `index.html` | Browser demo | 200+ |
| `example.js` | Node.js example | 50+ |
| Documentation | Guides & API refs | 500+ |

## ✅ What Works

- [x] Extract ICC profiles from PNG
- [x] Apply ICC profiles to PNG
- [x] Transfer profiles between images
- [x] Browser support with demo
- [x] Node.js support
- [x] TypeScript definitions
- [x] Error handling
- [x] Optimized binary size

## 🎓 Learn More

1. Start with `QUICK_START.md` for immediate usage
2. Review `index.html` for browser integration patterns
3. Check `example.js` for Node.js patterns
4. Read `WASM_README.md` for technical deep-dive
5. See `COMPLETE_SUMMARY.md` for full overview

## 📞 Support

For issues:
1. Verify input is valid PNG
2. Check console for detailed error messages
3. Review example files for patterns
4. Check documentation for API details
