// Types for arbitrage opportunities
export type Token = {
  address: string;
  symbol: string;
  decimals: number;
};

export type DEX = {
  name: string;
  chain: string;
};

export type ArbitrageOpportunity = {
  id: string;
  timestamp: number;
  sourceChain: string;
  targetChain: string;
  sourceToken: Token;
  targetToken: Token;
  sourceDex: DEX;
  targetDex: DEX;
  sourcePrice: number;
  targetPrice: number;
  profitPercentage: number;
  estimatedProfit: number;
  requiredAmount: number;
  gasEstimate: number;
}; 