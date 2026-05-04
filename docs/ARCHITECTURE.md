# 🏗️ VoltChain Architecture

VoltChain is a decentralized community microgrid energy trading platform built on the **Stellar** blockchain using **Soroban Smart Contracts**.

## 🧩 Components

### 1. Soroban Smart Contracts (`/contracts`)
- **EnergyTrade**: Handles the core logic for P2P energy sales. Now includes **persistent storage** for trade history and event emission for off-chain indexing.
- **MicrogridRegistry**: Manages community microgrid metadata and prosumer/consumer roles.
- **OracleConsumer**: Interfaces with off-chain smart meters to verify energy production/consumption.

### 2. Backend API (`/backend`)
- **Actix Web**: High-performance Rust web framework for the REST API.
- **PostgreSQL**: Stores persistent trade history using **Diesel ORM** for type-safe queries.
- **Redis**: Caching layer for real-time energy production data and session management.
- **Migrations**: Automated DB schema management for reliable deployments.

### 3. Frontend Dashboard (`/frontend`)
- **Next.js 14**: Modern React framework with App Router.
- **TailwindCSS**: Utility-first CSS for premium, responsive design.
- **Recharts**: Data visualization for energy consumption and trade trends.
- **Stellar SDK**: Client-side integration for signing transactions and querying the ledger.

## 🔄 Data Flow

1. **Smart Metering**: IoT devices send energy data to the Backend API.
2. **Verification**: The Backend API pushes data to an Oracle or directly interacts with Soroban if the data is signed by a trusted meter.
3. **Trading**: Prosumers list energy for sale; Consumers purchase via the Frontend.
4. **Settlement**: Soroban contracts execute the trade, transferring XLM/Tokens from Consumer to Prosumer and recording the kWh transfer on-chain.
