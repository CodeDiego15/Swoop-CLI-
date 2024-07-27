import React, { useState, useEffect } from 'react';
import './App.css';

function App() {
  const [input, setInput] = useState('');
  const [output, setOutput] = useState([]);

  const handleInputChange = (e) => {
    setInput(e.target.value);
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    if (input.trim()) {
      // Simular la ejecución de un comando (esto debería ser manejado por el backend)
      setOutput([...output, `> ${input}`]);
      setInput('');
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>My Terminal App</h1>
        <div className="terminal">
          <div className="output">
            {output.map((line, index) => (
              <div key={index}>{line}</div>
            ))}
          </div>
          <form onSubmit={handleSubmit}>
            <input
              type="text"
              value={input}
              onChange={handleInputChange}
              autoFocus
              placeholder="Type a command..."
            />
          </form>
        </div>
      </header>
    </div>
  );
}

export default App;
