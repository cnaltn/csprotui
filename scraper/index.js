import axios from 'axios';
import * as cheerio from 'cheerio';
import puppeteer from 'puppeteer';
import { BASE_URL } from './config.js';

function extractValue(text) {
  if (!text) return null;
  text = text.trim();
  if (text === '' || text === '-' || text === 'None') return null;
  return text;
}

function parseTableSection($, sectionId) {
  const result = {};
  const table = $(sectionId).find('table.settings');

  if (table.length === 0) return result;

  table.find('tr').each((_, row) => {
    const dataField = $(row).attr('data-field');
    const valueEl = $(row).find('td');

    if (dataField && valueEl.length) {
      const value = extractValue(valueEl.text());
      if (value) result[dataField] = value;
    }
  });

  return result;
}

async function scrapePlayer(playerSlug) {
  const url = `${BASE_URL}/${playerSlug}`;

  const [{ data }, browser] = await Promise.all([
    axios.get(url, {
      headers: {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
      }
    }),
    puppeteer.launch({
      headless: true,
      args: ['--no-sandbox', '--disable-setuid-sandbox', '--disable-dev-shm-usage', '--disable-extensions']
    })
  ]);

  const $ = cheerio.load(data);
  const playerName = extractValue($('h1').first().text()) || playerSlug;

  const page = await browser.newPage();
  await page.setRequestInterception(true);
  page.on('request', req => {
    if (['image', 'stylesheet', 'font', 'media'].includes(req.resourceType())) {
      req.abort();
    } else {
      req.continue();
    }
  });

  await page.goto(url, {
    waitUntil: 'domcontentloaded',
    timeout: 10000
  });

  // The first page load sometimes returns stale data (Cloudflare).
  // Reload once to get fresh content.
  await page.reload({ waitUntil: 'domcontentloaded', timeout: 10000 });

  const crosshairCode = await page.evaluate(() => {
    const pre = document.querySelector('#cs2_crosshair pre.js-csr-pre');
    return pre ? pre.textContent.trim() : null;
  }).catch(() => null);

  await browser.close();

  const result = {
    player: playerName,
    slug: playerSlug,
    url: url,
    data: {
      mouse: parseTableSection($, '#cs2_mouse'),
      crosshair: {
        importCode: crosshairCode,
        ...parseTableSection($, '#cs2_crosshair')
      },
      viewmodel: parseTableSection($, '#cs2_viewmodel'),
      video: parseTableSection($, '#cs2_video_settings'),
      radar: parseTableSection($, '#cs2_radar'),
      hud: parseTableSection($, '#cs2_hud'),
      bob: parseTableSection($, '#cs2_bob')
    }
  };

  const launchOpts = $('#cs2_launch_options').find('pre').text().trim();
  result.data.launchOptions = extractValue(launchOpts) || null;

  return result;
}

async function main() {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    console.log('Usage: npm run scrape <player-slug>');
    console.log('Example: npm run scrape xantares');
    console.log('\nExample players: xantares, m0NESY, ZywOo, s1mple, donk');
    process.exit(1);
  }

  const playerSlug = args[0];
  const result = await scrapePlayer(playerSlug);

  process.stdout.write(JSON.stringify(result));
}

export { scrapePlayer };
main();