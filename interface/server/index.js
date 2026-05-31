const path = require('path');
const express = require('express');
const { Pool } = require('pg');

const app = express();
const PORT = process.env.PORT || 80;

// Node.js backend talks to PostgreSQL directly (the same DB Actix writes into).
const pool = new Pool({
  host: process.env.PGHOST || 'localhost',
  port: Number(process.env.PGPORT) || 5432,
  user: process.env.PGUSER || 'admin_haha',
  password: process.env.PGPASSWORD || 'pasword_haha',
  database: process.env.PGDATABASE || 'db_haha',
});

// Mirror Actix's allowlist so the interpolated table name can't be abused.
const ALLOWED_TABLES = ['websocket', 'tcp', 'sm_cpp', 'sm_rust', 'mqtt'];

app.get('/api/:source/data', async (req, res) => {
  const source = req.params.source;
  if (!ALLOWED_TABLES.includes(source)) {
    return res.status(400).json({ error: `Unknown source '${source}'` });
  }

  try {
    const result = await pool.query(
      `SELECT timestamp, temperature, humidity, current FROM ${source} ORDER BY timestamp DESC LIMIT 100`
    );
    res.json(result.rows);
  } catch (e) {
    console.error(`[interface] DB query error for '${source}': ${e.message}`);
    res.status(500).json({ error: 'Failed to fetch data' });
  }
});

// Serve the built React frontend and let client-side routing fall back to index.html.
const buildDir = path.join(__dirname, 'build');
app.use(express.static(buildDir));
app.get('*', (_req, res) => {
  res.sendFile(path.join(buildDir, 'index.html'));
});

app.listen(PORT, () => {
  console.log(`[interface] Node.js backend + frontend listening on :${PORT}`);
});
