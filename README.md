# ⚡ VoltChain

**Empowering community microgrids through blockchain-based energy trading on Stellar**

![License](https://img.shields.io/badge/license-MIT-blue)
![Stellar](https://img.shields.io/badge/Stellar-Ready-blue)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)

---

## 🎯 The Vision

VoltChain addresses the **$850 billion energy access gap** for underserved communities. By leveraging the Stellar blockchain, we enable:

✅ **Peer-to-Peer Trading**: Direct energy sales between prosumers and consumers at fair market rates.
✅ **Smart Metering**: Verified consumption via Soroban Smart Contracts and oracle integration.
✅ **Near-Zero Fees**: Extremely low transaction costs with fast finality.
✅ **Green Certification**: On-chain renewable energy certificates (RECs) minted from verified solar/wind data.

---

## 🏗️ Architecture Stack

### Core Technology
- **Blockchain**: Stellar (Soroban Smart Contracts)
- **Backend API**: Rust (Actix Web, PostgreSQL, Redis, Diesel ORM)
- **Frontend**: Next.js 14 (React, TailwindCSS, Recharts)

### Repository Structure
```text
voltchain/
├── contracts/        # Soroban Smart Contracts (Rust)
├── backend/          # Rust Actix-Web API
├── frontend/         # Next.js Dashboard
├── scripts/          # Deployment & Management scripts
└── docs/             # Technical documentation
```
