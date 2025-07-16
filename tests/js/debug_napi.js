#!/usr/bin/env node
/**
 * Debug NAPI Module Loading for Node.js v22
 */

console.log('🔍 NAPI Debug Analysis for Node.js v22');
console.log('======================================');

console.log('\n📋 Environment Info:');
console.log('Node.js version:', process.version);
console.log('NAPI version:', process.versions.napi);
console.log('Modules ABI:', process.versions.modules);
console.log('Platform:', process.platform, process.arch);

// Test different loading strategies
console.log('\n🧪 Testing Module Loading Strategies:');

// Strategy 1: Direct require
console.log('\n1. Direct require:');
try {
    const binding = require('./index.node');
    console.log('✅ Success:', Object.keys(binding));
} catch (error) {
    console.log('❌ Failed:', error.message);
    console.log('   Error code:', error.code);
}

// Strategy 2: Check if file exists and permissions
console.log('\n2. File System Check:');
const fs = require('fs');
const path = require('path');

const bindingPath = path.resolve(__dirname, 'index.node');
console.log('Binding path:', bindingPath);

try {
    const stats = fs.statSync(bindingPath);
    console.log('✅ File exists, size:', stats.size, 'bytes');
    console.log('   Permissions:', stats.mode.toString(8));
    console.log('   Modified:', stats.mtime.toISOString());
} catch (error) {
    console.log('❌ File system error:', error.message);
}

// Strategy 3: Use dlopen if available
console.log('\n3. Dynamic Loading Analysis:');
try {
    const { Worker } = require('worker_threads');
    console.log('✅ Worker threads available');
    
    // Try to get more detailed error information
    const child_process = require('child_process');
    const result = child_process.spawnSync('ldd', [bindingPath], { encoding: 'utf8' });
    
    if (result.error) {
        console.log('   ldd not available');
    } else {
        console.log('   Dependencies:');
        result.stdout.split('\n').slice(0, 5).forEach(line => {
            if (line.trim()) console.log('   ', line.trim());
        });
    }
} catch (error) {
    console.log('❌ Dynamic loading analysis failed:', error.message);
}

// Strategy 4: Check binary format
console.log('\n4. Binary Format Check:');
try {
    const child_process = require('child_process');
    const fileResult = child_process.spawnSync('file', [bindingPath], { encoding: 'utf8' });
    
    if (fileResult.error) {
        console.log('   file command not available');
    } else {
        console.log('   Format:', fileResult.stdout.trim());
    }
    
    // Check for NAPI symbols
    const nmResult = child_process.spawnSync('nm', ['-D', bindingPath], { encoding: 'utf8' });
    if (!nmResult.error) {
        const symbols = nmResult.stdout.split('\n').filter(line => 
            line.includes('napi') || line.includes('init') || line.includes('register')
        );
        console.log('   NAPI-related symbols found:', symbols.length);
        symbols.slice(0, 5).forEach(symbol => {
            console.log('   ', symbol.trim());
        });
    }
} catch (error) {
    console.log('❌ Binary analysis failed:', error.message);
}

// Strategy 5: NAPI Version Compatibility Check
console.log('\n5. NAPI Compatibility Analysis:');
const nodeNapi = parseInt(process.versions.napi);
console.log('Node.js NAPI version:', nodeNapi);

// Check what NAPI versions are typically compatible
const knownCompatible = {
    8: ['v14', 'v16'],
    9: ['v16', 'v18'], 
    10: ['v18', 'v20', 'v22']
};

console.log('Expected compatible Node.js versions for NAPI', nodeNapi, ':', 
    knownCompatible[nodeNapi] || 'Unknown');

// Strategy 6: Alternative module loading
console.log('\n6. Alternative Loading Methods:');

// Try process.dlopen if available (Node.js internal)
if (process.dlopen) {
    try {
        const module = { exports: {} };
        process.dlopen(module, bindingPath);
        console.log('✅ process.dlopen succeeded:', Object.keys(module.exports));
    } catch (error) {
        console.log('❌ process.dlopen failed:', error.message);
    }
} else {
    console.log('   process.dlopen not available');
}

console.log('\n📝 Summary and Recommendations:');
console.log('1. Check if the binding was compiled with the correct NAPI version');
console.log('2. Verify the target Node.js ABI version matches');
console.log('3. Consider rebuilding with explicit NAPI version targeting');
console.log('4. Test with Node.js v18 for comparison');

console.log('\n💡 Possible Solutions:');
console.log('- Rebuild with: npm run build -- --target napi-v10');
console.log('- Update NAPI dependencies to latest stable');
console.log('- Use Node.js v18.x as a working baseline');
console.log('- Check for ABI compatibility issues');