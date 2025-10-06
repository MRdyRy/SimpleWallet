# 🦀 SimpleWallet — Clean Architecture Monorepo (Rust)
[![Build, Test](https://github.com/MRdyRy/SimpleWallet/actions/workflows/rust.yml/badge.svg)](https://github.com/MRdyRy/SimpleWallet/actions/workflows/rust.yml)

A **monorepo-style backend system** built in Rust to demonstrate **production-grade software engineering** with  
✅ Clean Architecture  
✅ Shared reusable libraries  
✅ Configurable via environment variables  
✅ Container & Kubernetes ready  

---

## 🧩 Overview

**SimpleWallet** simulates a distributed digital wallet system with multiple microservices:

| Service | Responsibility |
|----------|----------------|
| 🧍 **registration-service** | Handles user registration |
| 👤 **user-service** | Manages user information |
| 💰 **wallet-service** | Maintains wallet balance and updates |
| 🔁 **transfer-service** | Handles money transfers between users |
| 🧾 **receipt-service** | Generates PDF receipts for completed transfers |

---

## 🏗️ Monorepo Structure
```tree
├───domain
│   └───src
│       ├───base
│       ├───receipt
│       ├───transfer
│       ├───user
│       └───wallet
├───lib
│   └───src
│       ├───db
│       ├───http_client
│       └───log
└───services
    ├───receipt_service
    │   └───src
    ├───transfer_service
    │   └───src
    ├───user_service
    │   └───src
    └───wallet_service
        └───src
```
 
🧰 Tech Stack
|Category |	Stack |
|----------|---------------------|
|Language	| Rust 1.79+|
|Async runtime|	Tokio|
|HTTP Framework|	Axum|
|Database	|PostgreSQL (Deadpool Pool)|
|Cache	|Redis / Moka|
|HTTP Client	|Reqwest (Singleton + Retry)|
|Logging	|Tracing (JSON/Plain)|
|Architecture	|Clean Architecture|
|Deployment	|Docker → Kubernetes|
