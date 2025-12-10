#!/usr/bin/env node

/**
 * Node.js example for using shiny-png library
 * 
 * Usage:
 *   node example.js <source.png> <target.png> [output.png]
 *   node example.js --default <image.png> [output.png]
 */

import * as shinyPng from './pkg/shiny_png.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

async function main() {
  const args = process.argv.slice(2);
  
  if (args.length < 1) {
    console.error('Usage:');
    console.error('  node example.js <source.png> <target.png> [output.png]');
    console.error('  node example.js --default <image.png> [output.png]');
    console.error('\nExamples:');
    console.error('  node example.js source.png target.png result.png');
    console.error('  node example.js --default image.png result.png');
    process.exit(1);
  }

  try {
    if (args[0] === '--default') {
      // Apply default ICC profile
      if (args.length < 2) {
        console.error('Usage: node example.js --default <image.png> [output.png]');
        process.exit(1);
      }

      const inputFile = args[1];
      const outputFile = args[2] || 'result.png';

      console.log(`📖 Reading image: ${inputFile}`);
      const imageData = fs.readFileSync(inputFile);

      console.log('🎨 Applying default ICC profile...');
      const result = shinyPng.apply_default_icc_profile(imageData);
      console.log(`✓ Result image size: ${result.length} bytes`);

      console.log(`💾 Writing result to: ${outputFile}`);
      fs.writeFileSync(outputFile, result);

      console.log('✅ Success! Default ICC profile applied.');
      console.log(`\nResult saved to: ${path.resolve(outputFile)}`);

    } else {
      // Transfer ICC profile between images
      const sourceFile = args[0];
      const targetFile = args[1];
      const outputFile = args[2] || 'result.png';

      // Read the source image
      console.log(`📖 Reading source image: ${sourceFile}`);
      const sourceData = fs.readFileSync(sourceFile);

      // Extract ICC profile
      console.log('🎨 Extracting ICC profile from source...');
      const profile = shinyPng.extract_icc_profile(sourceData);
      
      if (profile.length === 0) {
        console.error('❌ Error: No ICC profile found in source image');
        process.exit(1);
      }
      console.log(`✓ Found ICC profile: ${profile.length} bytes`);

      // Read the target image
      console.log(`📖 Reading target image: ${targetFile}`);
      const targetData = fs.readFileSync(targetFile);

      // Apply the profile
      console.log('🔄 Applying ICC profile to target image...');
      const result = shinyPng.apply_icc_profile(targetData, profile);
      console.log(`✓ Result image size: ${result.length} bytes`);

      // Write the output
      console.log(`💾 Writing result to: ${outputFile}`);
      fs.writeFileSync(outputFile, result);

      console.log('✅ Success! ICC profile transferred.');
      console.log(`\nResult saved to: ${path.resolve(outputFile)}`);
    }

  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

main();
