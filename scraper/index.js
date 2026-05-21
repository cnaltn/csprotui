import axios from 'axios';
import * as cheerio from 'cheerio';
import { BASE_URL } from './config.js';

const REQUEST_TIMEOUT_MS = 15000;

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

function classifyHttpError(status) {
  if (status === 404) return 'PLAYER_NOT_FOUND';
  if (status === 403 || status === 429) return 'ACCESS_BLOCKED';
  if (status >= 500) return 'SERVER_ERROR';
  return 'FETCH_FAILED';
}

async function scrapePlayer(playerSlug) {
  const url = `${BASE_URL}/${playerSlug}`;

  let data;
  try {
    const response = await axios.get(url, {
      headers: {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
      },
      timeout: REQUEST_TIMEOUT_MS,
      maxRedirects: 5
    });
    data = response.data;
  } catch (err) {
    if (err.code === 'ECONNABORTED') {
      throw new Error(JSON.stringify({ error: 'TIMEOUT', message: 'Request timed out.' }));
    }
    if (err.response) {
      const code = classifyHttpError(err.response.status);
      throw new Error(JSON.stringify({
        error: code,
        message: `HTTP ${err.response.status} from prosettings.net.`
      }));
    }
    if (err.code === 'ENOTFOUND' || err.code === 'ECONNREFUSED') {
      throw new Error(JSON.stringify({
        error: 'NETWORK_ERROR',
        message: 'Cannot reach prosettings.net. Check your internet connection.'
      }));
    }
    throw new Error(JSON.stringify({
      error: 'FETCH_FAILED',
      message: err.message || 'Unknown network error.'
    }));
  }

  const $ = cheerio.load(data);
  const playerName = extractValue($('h1').first().text()) || playerSlug;

  const result = {
    player: playerName,
    slug: playerSlug,
    url: url,
    data: {
      mouse: parseTableSection($, '#cs2_mouse'),
      crosshair: {
        importCode: null,
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

  try {
    const result = await scrapePlayer(playerSlug);
    process.stdout.write(JSON.stringify(result));
  } catch (err) {
    // Write structured error JSON to stderr so Rust can parse it
    let errorJson = err.message;
    try {
      JSON.parse(errorJson); // validate
    } catch {
      errorJson = JSON.stringify({ error: 'UNKNOWN', message: err.message || 'Unknown scraper error.' });
    }
    process.stderr.write(errorJson);
    process.exit(1);
  }
}

export { scrapePlayer };
main();
