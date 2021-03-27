import { useEffect, useState } from 'react';
import './App.css';
import axios from 'axios';
import { config } from './Config';

export default function App() {
  const [name, setName] = useState("");

  useEffect(() => {
    async function fetchData() {
      // TODO: this should not be hardcoded, and should vary between
      // dev & production.
      const response = await axios(config.urls.BASE_URL + "/api/name")
      setName(response.data)
    }
    fetchData()
  }, []);

  return <div className="App-Div">Hello {name}</div>;
}
