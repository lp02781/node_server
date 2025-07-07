import React, { useEffect, useState } from 'react';
import { TopNav } from './components/TopNav';
import { ViewToggle } from './components/ViewToggle';
import SensorTable from './components/SensorTable';
import SensorGraph from './components/SensorGraph';

import { SensorData } from './types/SensorData';

const App: React.FC = () => {
  const [source, setSource] = useState('sm_cpp');
  const [view, setView] = useState<'table' | 'graph'>('table');
  const [data, setData] = useState<SensorData[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`http://localhost:5000/db/${source}/data`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const json = await res.json();
      setData(json);
    } catch (e: any) {
      setError(e.message || 'Fetch error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, [source]);

  return (
    <div style={{ padding: '2rem', position: 'relative' }}>
      <TopNav selected={source} onSelect={setSource} />
      <ViewToggle view={view} onChange={setView} />
      <h2>{source.toUpperCase()} - {view.toUpperCase()} View</h2>

      {loading && <p>Loading...</p>}
      {error && <p style={{ color: 'red' }}>Error: {error}</p>}

      {!loading && !error && view === 'table' && <SensorTable data={data} />}
      {!loading && !error && view === 'graph' && <SensorGraph />}
    </div>
  );
};

export default App;
