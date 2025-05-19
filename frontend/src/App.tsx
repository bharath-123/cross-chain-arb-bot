import React, { useEffect, useState } from 'react';
import type { ArbitrageOpportunity } from './types/arbitrage';
import { websocketService } from './services/websocket';
import { ArbitrageTable } from './components/ArbitrageTable';

function App() {
  const [opportunities, setOpportunities] = useState<ArbitrageOpportunity[]>([]);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    websocketService.connect();
    setIsConnected(true);

    const unsubscribe = websocketService.subscribe((opportunity) => {
      setOpportunities((prev) => [opportunity, ...prev].slice(0, 100)); // Keep last 100 opportunities
    });

    return () => {
      unsubscribe();
      websocketService.disconnect();
    };
  }, []);

  return (
    <div className="min-h-screen bg-gray-100">
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
          <h1 className="text-3xl font-bold text-gray-900">Cross-Chain Arbitrage Opportunities</h1>
          <div className="mt-2">
            <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
              isConnected ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
            }`}>
              {isConnected ? 'Connected' : 'Disconnected'}
            </span>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-white shadow rounded-lg">
            <div className="px-4 py-5 sm:p-6">
              <ArbitrageTable opportunities={opportunities} />
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
