Here is a tailored, production-ready `README.md` formatted for your project repository.

---

```markdown
# 🛡️ Aegis Vault

A modern, fast, and secure desktop authenticator application built with **Vue 3**, **TypeScript**, and **Tauri** (Rust). Aegis Vault brings offline 2FA token management to your desktop with local hardware binding and zero-knowledge architecture.

---

## ✨ Features

- 🔒 **Zero-Knowledge Encrypted Vault**: Master password key derivation via **Argon2id** and **AES-256-GCM** encryption.
- 🪟 **Hardware-Bound Security**: Windows DPAPI integration protects vault files at the OS level.
- ⏱️ **Live OTP Token Generation**: Real-time TOTP/HOTP calculation with dynamic visual timers (RFC 6238 / RFC 4226).
- 📷 **Integrated QR Scanner**: Fast camera scanning and manual `otpauth://` URI parsing.
- 🌙 **Modern Cyberpunk UI**: Built with Tailwind CSS using a Neon Blue and Soft Pink palette.
- 💤 **Auto-Lock Timeout**: Inactivity detection locks the vault after 3 minutes of idle time.
- 🌐 **100% Offline & Private**: Zero background tracking, telemetry, or cloud dependencies.

---

## 🛠️ Tech Stack

### Frontend
- **Framework**: [Vue 3](https://vuejs.org/) (Composition API + `<script setup>`)
- **State Management**: [Pinia](https://pinia.vuejs.org/)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)
- **Icons**: [Lucide Vue Next](https://lucide.dev/)
- **Scanner**: `html5-qrcode`

### Backend
- **Framework**: [Tauri v2](https://tauri.app/) (Rust)
- **Cryptography**: Argon2, AES-GCM, Windows DPAPI
- **Hashing**: HMAC-SHA1, HMAC-SHA256, HMAC-SHA512
- **Memory Safety**: Automated RAM key zeroization (`zeroize`)

---

## 📁 Project Structure

```text
├── src/                          # Vue 3 Frontend
│   ├── components/               # Reusable UI Components
│   │   ├── AccountCard.vue       # Token display & progress ring
│   │   ├── AccountForm.vue       # Manual secret entry form
│   │   ├── QRScanner.vue         # Camera QR scanner
│   │   └── UnlockModal.vue       # Master password entry overlay
│   ├── services/                 # IPC & Token Utilities
│   │   ├── otpUri.ts             # otpauth:// URI parser
│   │   ├── totp.ts               # Base32 & frontend helper
│   │   └── vault.ts              # Tauri IPC bridge
│   ├── stores/                   # Global Pinia state engine
│   │   └── authenticator.ts
│   ├── views/                    # Main App Views
│   │   ├── Home.vue              # Main account list
│   │   ├── AddAccount.vue        # QR/Manual entry view
│   │   └── Settings.vue          # Vault security info
│   ├── App.vue                   # Core layout & idle timer
│   ├── main.ts                   # Vue app initialization
│   └── style.css                 # Custom design tokens & theme
│
└── src-tauri/                    # Rust Backend
    ├── src/
    │   ├── commands/             # Tauri IPC Command Handlers
    │   ├── crypto/               # Key derivation, AES-GCM & DPAPI
    │   ├── models/               # Data structures
    │                             # HMAC dynamic truncation engine
    │                             # Thread-safe encrypted file persistence
    │   ├── lib.rs                # Module declarations
    │   └── main.rs               # Tauri app lifecycle entry
    ├── Cargo.toml
    └── tauri.conf.json

```

---

## 🚀 Getting Started

### Prerequisites

1. **Node.js**: `v18+` or `v20+` recommended.
2. **Rust**: Install Rust via [rustup.rs](https://rustup.rs/).
3. **C++ Build Tools**: Install *Desktop development with C++* via Visual Studio Installer (Windows).

### Installation

Clone the repository and install dependencies:

```bash
git clone [https://github.com/your-username/aegis-vault.git](https://github.com/your-username/aegis-vault.git)
cd aegis-vault
npm install

```

### Development Mode

Run the app in live-reload development mode:

```bash
npm run tauri dev

```

### Building for Production

Compile a native executable for your operating system:

```bash
npm run tauri build

```

The output binary will be generated in `src-tauri/target/release/`.

---

## 🛡️ Security Architecture

1. **Master Password KDF**: Passwords are stretched using **Argon2id** (`m=65536, t=3, p=4`) to produce a 256-bit key.
2. **Vault Encryption**: Stored tokens are serialized to JSON and encrypted using **AES-256-GCM**.
3. **Hardware Binding**: On Windows, the encrypted file is wrapped with **DPAPI** (`CryptProtectData`), binding decryption capabilities to the current user profile.
4. **RAM Protection**: Master key buffers are wrapped in `zeroize` containers and automatically erased from memory when the vault locks or the application terminates.

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for details.

```
