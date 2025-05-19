import type { ArbitrageOpportunity, Token, DEX } from '../types/arbitrage';

const CHAINS = ['Ethereum', 'Unichain'];
const DEXES: DEX[] = [
  { name: 'Uniswap V3', chain: 'Ethereum' },
  { name: 'SushiSwap', chain: 'Ethereum' },
  { name: 'UniDex', chain: 'Unichain' },
  { name: 'PancakeSwap', chain: 'Unichain' },
];

const TOKENS: Token[] = [
  { address: '0x1', symbol: 'ETH', decimals: 18 },
  { address: '0x2', symbol: 'USDC', decimals: 6 },
  { address: '0x3', symbol: 'USDT', decimals: 6 },
  { address: '0x4', symbol: 'DAI', decimals: 18 },
];

function generateRandomPrice(): number {
  return Math.random() * 1000 + 100; // Random price between 100 and 1100
}

function generateRandomAmount(): number {
  return Math.random() * 10000 + 1000; // Random amount between 1000 and 11000
}

function generateRandomProfit(): number {
  return (Math.random() * 5 - 0.5); // Random profit between -0.5% and 4.5%
}

export function generateMockOpportunity(): ArbitrageOpportunity {
  const sourceChain = CHAINS[Math.floor(Math.random() * CHAINS.length)];
  const targetChain = CHAINS.find(chain => chain !== sourceChain)!;
  
  const sourceDex = DEXES.find(dex => dex.chain === sourceChain)!;
  const targetDex = DEXES.find(dex => dex.chain === targetChain)!;
  
  const sourceToken = TOKENS[Math.floor(Math.random() * TOKENS.length)];
  const targetToken = TOKENS[Math.floor(Math.random() * TOKENS.length)];

  const sourcePrice = generateRandomPrice();
  const profitPercentage = parseFloat(generateRandomProfit().toFixed(2));
  const targetPrice = sourcePrice * (1 + profitPercentage / 100);
  const requiredAmount = generateRandomAmount();
  const estimatedProfit = (requiredAmount * profitPercentage) / 100;
  const gasEstimate = Math.random() * 0.1 + 0.01; // Random gas estimate between 0.01 and 0.11 ETH

  return {
    id: Math.random().toString(36).substring(7),
    timestamp: Date.now(),
    sourceChain,
    targetChain,
    sourceToken,
    targetToken,
    sourceDex,
    targetDex,
    sourcePrice,
    targetPrice,
    profitPercentage,
    estimatedProfit,
    requiredAmount,
    gasEstimate,
  };
} 