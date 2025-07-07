import React, { useEffect, useState } from 'react';
import axios from 'axios';
import './App.css';

type SensorData = {
  timestamp: number;
  temperature: number;
  humidity: number;
  current: number;
};

const App: React.FC = () => {
  const [data, setData] = useState<SensorData[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    axios.get<SensorData[]>('http://localhost:5000/db/sm_cpp/data')
      .then(res => {
        setData(res.data);
        setLoading(false);
      })
      .catch(err => {
        console.error("Failed to fetch sensor data:", err);
        setLoading(false);
      });
  }, []);

  return (
    <div className="App">
      <h1>SM_CPP Sensor Data</h1>
      {loading ? (
        <p>Loading...</p>
      ) : (
        <table>
          <thead>
            <tr>
              <th>Timestamp</th>
              <th>Temperature</th>
              <th>Humidity</th>
              <th>Current</th>
            </tr>
          </thead>
          <tbody>
            {data.map((row, i) => (
              <tr key={i}>
                <td>{row.timestamp.toFixed(2)}</td>
                <td>{row.temperature.toFixed(2)}</td>
                <td>{row.humidity}</td>
                <td>{row.current.toFixed(2)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
};

export default App;
