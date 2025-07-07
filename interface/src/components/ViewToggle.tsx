import React from 'react';

type Props = {
  view: 'table' | 'graph';
  onChange: (view: 'table' | 'graph') => void;
};

export const ViewToggle: React.FC<Props> = ({ view, onChange }) => (
  <div style={{ position: 'absolute', right: 20, top: 20 }}>
    <button onClick={() => onChange('table')} disabled={view === 'table'}>Table</button>
    <button onClick={() => onChange('graph')} disabled={view === 'graph'} style={{ marginLeft: '1rem' }}>Graph</button>
  </div>
);
