# Solana API Documentation

Base URL: `http://localhost:8000`

All endpoints are prefixed with `/solana`

## Response Format

All endpoints return responses in the following format:

```json
{
  "success": boolean,
  "data": object | null,
  "error": string | null
}
```

## Endpoints

### 1. Health Check

**GET** `/solana/health`

Check if the API service is running.

#### Response
```json
{
  "success": true,
  "data": "OK",
  "error": null
}
```

---

### 2. Get Balance

**POST** `/solana/balance`

Get SOL balance for a specific public key.

#### Request Body
```json
{
  "pubkey": "string" // Solana public key
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "pubkey": "string",
    "balance": 1.23456789,
    "token": "SOL"
  },
  "error": null
}
```

---

### 3. Prepare Swap Transaction

**POST** `/solana/swap/prepare`

Prepare an unsigned swap transaction using Jupiter.

#### Request Body
```json
{
  "payer_pubkey": "string", // Public key that pays fees
  "from_token": "string",   // Token symbol or mint address
  "to_token": "string",     // Token symbol or mint address
  "amount": 1.5             // Amount to swap
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "unsigned_transaction": "string", // Base64 encoded unsigned transaction
    "quote_info": {
      "expected_output": 150.25,
      "price_impact": 0.12,
      "route_steps": 1
    },
    "required_signers": ["string"],   // Array of required signer pubkeys
    "recent_blockhash": "string"
  },
  "error": null
}
```

---

### 4. Prepare SOL Transfer Transaction

**POST** `/solana/transaction/prepare`

Prepare an unsigned SOL transfer transaction.

#### Request Body
```json
{
  "payer_pubkey": "string", // Public key that pays fees and sends
  "to_address": "string",   // Recipient public key
  "amount": 1.0             // Amount in SOL
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "unsigned_transaction": "string", // Base64 encoded unsigned transaction
    "from": "string",
    "to": "string",
    "amount": 1.0,
    "required_signers": ["string"],
    "recent_blockhash": "string"
  },
  "error": null
}
```

---

### 5. Submit Signed Transaction

**POST** `/solana/transaction/submit`

Submit a signed transaction to the Solana network.

#### Request Body
```json
{
  "signed_transaction": "string" // Base64 encoded signed transaction
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "signature": "string",
    "status": "submitted"
  },
  "error": null
}
```

---

### 6. Get Token Price

**POST** `/solana/price`

Get current USD price for a token.

#### Request Body
```json
{
  "token": "string" // Token symbol or mint address
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "token": "SOL",
    "price": 123.45,
    "currency": "USD"
  },
  "error": null
}
```

---

### 7. Search Tokens

**POST** `/solana/tokens/search`

Search for tokens by symbol, name, or address.

#### Request Body
```json
{
  "query": "string" // Search term
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "tokens": [
      {
        "symbol": "RAY",
        "name": "Raydium",
        "address": "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R",
        "decimals": 6
      }
    ],
    "count": 1
  },
  "error": null
}
```

---

### 8. Get Wallet Tokens

**POST** `/solana/wallet/tokens`

Get all tokens held by a wallet with balances and USD values.

#### Request Body
```json
{
  "pubkey": "string" // Wallet public key
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "pubkey": "string",
    "tokens": [
      {
        "symbol": "SOL",
        "name": "Solana",
        "mint": "So11111111111111111111111111111111111111112",
        "balance": 5.123456789,
        "decimals": 9,
        "usd_value": 632.17
      },
      {
        "symbol": "USDC",
        "name": "USD Coin",
        "mint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "balance": 100.0,
        "decimals": 6,
        "usd_value": 100.0
      }
    ],
    "total_tokens": 2
  },
  "error": null
}
```

---

### 9. Get Transaction History

**POST** `/solana/transactions/history`

Get transaction history for a wallet.

#### Request Body
```json
{
  "pubkey": "string",          // Wallet public key
  "limit": 50,                 // Optional: number of transactions (default: 50)
  "before": "string"           // Optional: signature to paginate before
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "pubkey": "string",
    "transactions": [
      {
        "signature": "string",
        "status": "Success",       // "Success" | "Failed" | "Pending"
        "confirmation_status": "Finalized", // "Processed" | "Confirmed" | "Finalized"
        "block_time": 1640995200,  // Unix timestamp
        "slot": 123456789,
        "fee": 0.000005,          // SOL
        "amount": 1.5,            // Amount transferred
        "token_symbol": "SOL",
        "transaction_type": "Transfer", // "Transfer" | "TokenTransfer" | "Swap" | "Unknown"
        "error": null
      }
    ],
    "total_count": 1,
    "has_more": false,
    "next_before": null
  },
  "error": null
}
```

---

### 10. Get Pending Transactions

**POST** `/solana/transactions/pending`

Get pending transactions for a wallet.

#### Request Body
```json
{
  "pubkey": "string" // Wallet public key
}
```

#### Response
```json
{
  "success": true,
  "data": {
    "pubkey": "string",
    "pending_transactions": [
      {
        "signature": "string",
        "status": "Pending",
        "confirmation_status": "Processed",
        "block_time": 1640995200,
        "slot": 123456789,
        "fee": null,
        "amount": 1.0,
        "token_symbol": "SOL",
        "transaction_type": "Transfer",
        "error": null
      }
    ],
    "count": 1
  },
  "error": null
}
```

## Error Responses

When an error occurs, the response will have `success: false` and include an error message:

```json
{
  "success": false,
  "data": null,
  "error": "Error description here"
}
```

## Common Error Types

- **Invalid public key**: Malformed Solana public key
- **Insufficient balance**: Not enough tokens for the operation
- **Network error**: Connection issues with Solana RPC
- **Transaction failed**: Transaction simulation or execution failed
- **Token not found**: Unknown token symbol or mint address

## Notes

- All amounts are in their native token units (SOL uses 9 decimals, USDC uses 6 decimals)
- Transactions must be signed client-side before submission
- The API runs on devnet by default (configurable)
- USD values are fetched from Jupiter's price API and may be `null` if unavailable