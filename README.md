# SOON Network SPL Token

A secure and efficient SPL token implementation for the SOON Network blockchain.

## Features

- Token initialization with customizable parameters
- Minting capabilities with authority controls
- Secure token transfers
- Token burning functionality
- Vesting mechanism for token distribution
- Comprehensive test suite
- SOON Network integration

## Prerequisites

- Rust 1.70.0 or later
- Cargo
- SOON Network CLI tools
- Anchor Framework 0.29.0

## Installation

```bash
# Clone the repository
git clone https://github.com/your-username/soon-spl-token.git
cd soon-spl-token

# Build the project
cargo build
```

## Usage

### Initialize a New Token

```rust
let args = InitializeTokenArgs {
    name: "My Token".to_string(),
    symbol: "MTK".to_string(),
    decimals: 9,
    total_supply: 1_000_000_000,
};

// Initialize token
initialize_token(ctx, args)?;
```

### Mint Tokens

```rust
let args = MintToArgs {
    amount: 1000,
};

// Mint tokens to an account
mint_to(ctx, args)?;
```

### Transfer Tokens

```rust
let args = TransferArgs {
    amount: 100,
};

// Transfer tokens between accounts
transfer(ctx, args)?;
```

## Testing

Run the test suite:

```bash
cargo test
```

## Security Considerations

- All token operations require proper authority verification
- Vesting schedules are immutable once set
- Transfer amounts are validated against account balances
- Minting is controlled by authorized accounts only

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For support, please open an issue in the GitHub repository or contact the SOON Network team.
