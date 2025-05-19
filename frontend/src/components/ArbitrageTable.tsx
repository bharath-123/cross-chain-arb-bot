import React from 'react';
import type { ArbitrageOpportunity } from '../types/arbitrage';
import { formatNumber } from '../utils/format';

interface ArbitrageTableProps {
  opportunities: ArbitrageOpportunity[];
}

export const ArbitrageTable: React.FC<ArbitrageTableProps> = ({ opportunities }) => {
  return (
    <div className="overflow-x-auto">
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Time</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Source Chain</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Target Chain</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Token Pair</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Source DEX</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Target DEX</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Source Price</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Target Price</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Profit %</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Est. Profit</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Required Amount</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Gas Est.</th>
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {opportunities.map((opp) => (
            <tr key={opp.id} className="hover:bg-gray-50">
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {new Date(opp.timestamp).toLocaleTimeString()}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{opp.sourceChain}</td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{opp.targetChain}</td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {opp.sourceToken.symbol}/{opp.targetToken.symbol}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{opp.sourceDex.name}</td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{opp.targetDex.name}</td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                ${formatNumber(opp.sourcePrice)}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                ${formatNumber(opp.targetPrice)}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm">
                <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                  opp.profitPercentage > 0 ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                }`}>
                  {formatNumber(opp.profitPercentage)}%
                </span>
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                ${formatNumber(opp.estimatedProfit)}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                ${formatNumber(opp.requiredAmount)}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {formatNumber(opp.gasEstimate)} ETH
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}; 