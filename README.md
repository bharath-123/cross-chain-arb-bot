# Tycho Cross Chain Arbitrage Bot

This is a monorepo containing both the backend arbitrage bot and frontend dashboard for the Tycho Cross Chain Arbitrage Bot.

## Project Structure

- `backend/`: Rust-based arbitrage bot that checks for cross-chain arbitrage opportunities
- `frontend/`: React-based dashboard for monitoring and controlling the arbitrage bot

## Setup Instructions

### Backend (Rust)

```bash
cd backend
cargo build
```

### Frontend (React)

```bash
cd frontend
npm install
npm run dev
```

## Development

- The backend is written in Rust and uses Cargo for dependency management
- The frontend is built with React + TypeScript using Vite as the build tool
- Both projects can be developed independently in their respective directories 