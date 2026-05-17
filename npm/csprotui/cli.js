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

const scraperDir = path.join(__dirname, 'scraper');
const env = { ...process.env };
if (fs.existsSync(scraperDir)) {
  env.CSPROTUI_SCRAPER_DIR = scraperDir;
}

const result = spawnSync(binary, process.argv.slice(2), {
  stdio: 'inherit',
  shell: false,
  env,
});

process.exit(result.status ?? 1);
