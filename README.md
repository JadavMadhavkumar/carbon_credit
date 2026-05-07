# Falkon Future X Carbon Credit Platform - Rust Backend v2.0

## Overview

This is a production-grade Rust backend for calculating carbon credits from household waste management. The system implements scientifically-grounded carbon calculation formulas based on IPCC AR6, EPA WARM, and Indian regulatory standards.

## Carbon Calculation System

### Master Formula
```
Credits (kg CO₂e) = Weight × Emission Factor × Multiplier × Verification Score × Locality Factor × Quality Factor
```

### Key Emission Factors

| Waste Type | EF (kg CO₂e/kg) | Multiplier |
|------------|-----------------|------------|
| PET Bottles | 1.78 | 2.1x |
| Aluminum Cans | 8.14 | 6.0x |
| Mobile Phones | 45.0 | 8.0x |
| Laptop | 320.0 | 10.0x |
| Cotton Clothing | 6.80 | 3.0x |
| Vegetable Scraps | 0.57 | 1.0x |

### Biochar Carbon Sequestration
```
Carbon Removed = Biomass Input × Yield Rate × Carbon Content × Stability Factor
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/v1/carbon/emission | Calculate emission for waste |
| POST | /api/v1/carbon/credit | Calculate carbon credits |
| POST | /api/v1/waste/submit | Submit waste for credit calculation |
| POST | /api/v1/biochar/process | Process biomass to biochar |
| GET | /api/v1/analytics | Platform analytics |

## Setup

```bash
# Install dependencies
cargo build

# Run database migrations
cargo run

# Set environment variables
export DATABASE_URL=postgres://user:pass@localhost/falkon_carbon
export SERVER_PORT=3000
export JWT_SECRET=your-secret-key
```

## Database Schema

- **users**: User accounts with wallet addresses
- **households**: Households linked to users and states
- **waste_submissions**: Waste submissions with verification scores
- **credit_ledger**: Carbon credit earn/redeem transactions
- **biochar_records**: Biochar processing records
- **reports**: Aggregated reports by period

## Running

```bash
cargo run
```

The server will start on http://localhost:3000