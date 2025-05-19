import { io, Socket } from 'socket.io-client';
import type { ArbitrageOpportunity } from '../types/arbitrage';
import { generateMockOpportunity } from './mockData';

class WebSocketService {
  private socket: Socket | null = null;
  private subscribers: ((opportunity: ArbitrageOpportunity) => void)[] = [];
  private mockInterval: number | null = null;

  connect() {
    // In development, use mock data
    if (import.meta.env.DEV) {
      console.log('Using mock data in development mode');
      this.startMockData();
      return;
    }

    this.socket = io('ws://localhost:8080');

    this.socket.on('connect', () => {
      console.log('Connected to WebSocket server');
    });

    this.socket.on('arbitrage_opportunity', (opportunity: ArbitrageOpportunity) => {
      this.subscribers.forEach(callback => callback(opportunity));
    });

    this.socket.on('disconnect', () => {
      console.log('Disconnected from WebSocket server');
    });
  }

  private startMockData() {
    // Generate a new opportunity every 2 seconds
    this.mockInterval = window.setInterval(() => {
      const opportunity = generateMockOpportunity();
      this.subscribers.forEach(callback => callback(opportunity));
    }, 2000);
  }

  subscribe(callback: (opportunity: ArbitrageOpportunity) => void) {
    this.subscribers.push(callback);
    return () => {
      this.subscribers = this.subscribers.filter(cb => cb !== callback);
    };
  }

  disconnect() {
    if (this.socket) {
      this.socket.disconnect();
      this.socket = null;
    }
    if (this.mockInterval) {
      clearInterval(this.mockInterval);
      this.mockInterval = null;
    }
  }
}

export const websocketService = new WebSocketService(); 