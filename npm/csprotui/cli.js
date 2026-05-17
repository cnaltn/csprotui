#!/usr/bin/env node
const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const BIN_DIR = path.join(__dirname, 'bin');
const ext = process.platform === 'win32' ? '.exe' : '';
const binary = path.join(BIN_DIR, `csprotui${ext}`);

if (!fs.existsSync(binary)) {
  console.error('csprotui binary not found. Run: npm install -g csprotui');
  process.exit(1);
}

const env = { ...process.env };

// Set scraper directory
const scraperDir = path.join(__dirname, 'scraper');
if (fs.existsSync(scraperDir)) {
  env.CSPROTUI_SCRAPER_DIR = scraperDir;
}

// Base64-encoded URL — replaced at publish time by CI
const ENCODED_URL = '__ENCODED_URL__';
if (ENCODED_URL !== '__ENCODED_URL__') {
  env.CSPROTUI_BASE_URL = Buffer.from(ENCODED_URL, 'base64').toString('utf8');
} else if (!env.CSPROTUI_BASE_URL) {
  console.error('Config error: CSPROTUI_BASE_URL not set and no embedded URL found.');
  process.exit(1);
}

const result = spawnSync(binary, process.argv.slice(2), {
  stdio: 'inherit',
  shell: false,
  env,
});

process.exit(result.status ?? 1);
