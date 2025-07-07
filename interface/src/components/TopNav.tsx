import React from 'react';

type Props = {
  selected: string;
  onSelect: (source: string) => void;
};

const sources = ['tcp', 'sm_cpp', 'sm_rust', 'mqtt', 'websocket'];

export const TopNav: React.FC<Props> = ({ selected, onSelect }) => {
  return (
    <div style={{ display: 'flex', gap: '1rem', marginBottom: '1rem' }}>
      {sources.map((src) => (
        <button
          key={src}
          onClick={() => onSelect(src)}
          style={{
            backgroundColor: src === selected ? '#007BFF' : '#EEE',
            color: src === selected ? '#FFF' : '#000',
            padding: '8px 12px',
            border: '1px solid #ccc',
            borderRadius: 4,
            cursor: 'pointer'
          }}
        >
          {src.toUpperCase()}
        </button>
      ))}
    </div>
  );
};
