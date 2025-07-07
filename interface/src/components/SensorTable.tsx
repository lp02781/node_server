import React from 'react';
import { SensorData } from '../types/SensorData';

type Props = {
  data: SensorData[];
};

const SensorTable: React.FC<Props> = ({ data }) => (
  <table style={{ borderCollapse: 'collapse', width: '100%' }}>
    <thead style={{ backgroundColor: '#f0f0f0' }}>
      <tr>
        <th style={{ border: '1px solid #ddd', padding: '8px' }}>Timestamp</th>
        <th style={{ border: '1px solid #ddd', padding: '8px' }}>Temperature</th>
        <th style={{ border: '1px solid #ddd', padding: '8px' }}>Humidity</th>
        <th style={{ border: '1px solid #ddd', padding: '8px' }}>Current</th>
      </tr>
    </thead>
    <tbody>
      {data.map((row, i) => (
        <tr key={i}>
          <td style={{ border: '1px solid #ddd', padding: '8px' }}>{row.timestamp.toFixed(2)}</td>
          <td style={{ border: '1px solid #ddd', padding: '8px' }}>{row.temperature.toFixed(2)}</td>
          <td style={{ border: '1px solid #ddd', padding: '8px' }}>{row.humidity}</td>
          <td style={{ border: '1px solid #ddd', padding: '8px' }}>{row.current.toFixed(2)}</td>
        </tr>
      ))}
    </tbody>
  </table>
);

export default SensorTable;
