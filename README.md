# Building Polkadot Blockchain
Building Polkadot Blockchain with macros in Rust

# Building Polkadot Blockchain

This project demonstrates how to clean up the Proof of Existence Pallet using the `#[macros::call]` attribute. This approach eliminates redundant code, simplifies the implementation, and improves maintainability.

## Overview

We previously used the `#[macros::call]` macro in the Balances Pallet to streamline its implementation. Here, we'll extend the same principle to the Proof of Existence Pallet to:

1. Replace verbose code with concise macro-driven logic.
2. Automate the generation of `Call` and `Dispatch` logic.
3. Improve the overall developer experience.

## Steps to Apply the Macro

Follow these steps to update the Proof of Existence Pallet:

### 1. Add the `#[macros::call]` Attribute

- Move the `create_claim` and `revoke_claim` functions into their own `impl<T: Config> Pallet<T>` block.
- Add the `#[macros::call]` attribute above this implementation.

```rust
#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err(&"this content is already claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
        if caller != *owner {
            return Err(&"this content is owned by someone else");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}
```

### 2. Remove Redundant Code

- Delete the existing `enum Call`.
- Delete the existing implementation of `Dispatch` for `Pallet`.

### 3. Update `main.rs`

Update the function calls in your `main.rs` file:

- Change `proof_of_existence::Call::CreateClaim` to `proof_of_existence::Call::create_claim` (use snake_case).
- Change `proof_of_existence::Call::RevokeClaim` to `proof_of_existence::Call::revoke_claim`.

Example:

```rust
let block_2 = types::Block {
    header: support::Header { block_number: 2 },
    extrinsics: vec![
        support::Extrinsic {
            caller: alice.clone(),
            call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                claim: &"Hello, world!",
            }),
        },
        support::Extrinsic {
            caller: bob.clone(),
            call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                claim: &"Hello, world!",
            }),
        },
    ],
};
```

### 4. Test the Changes

- Compile and run the project to ensure everything works as expected.
- The functionality should remain unchanged, but the code is now cleaner and easier to maintain.

## Exploring Macro-Generated Code

To understand how macros expand into Rust code, use the `cargo expand` tool:

```bash
cargo expand > out.rs
```

This generates a file, `out.rs`, with the expanded code. Key observations:

1. All `mod` files are combined into a single file.
2. The `#[macros::call]` attribute generates the necessary `Call` and `Dispatch` logic.
3. Other macros like `vec![]`, `println!()`, and `#[derive(Debug)]` are expanded into their full implementations.

### Example of Expanded Code

```rust
#[automatically_derived]
impl<T: ::core::fmt::Debug + Config> ::core::fmt::Debug for Pallet<T>
where
    T::Content: ::core::fmt::Debug,
    T::AccountId: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Pallet", "claims", &&self.claims)
    }
}
```

### Takeaways

1. Macros generate regular Rust code and follow the same rules as handwritten code.
2. They simplify development while adhering to Rust's strict type system.
3. Use `cargo expand` to inspect macro-generated code and deepen your understanding of macro behavior.

## Conclusion

By applying the `#[macros::call]` attribute, the Proof of Existence Pallet becomes:

- More concise.
- Easier to maintain.
- In line with Rust's powerful macro system.

Leverage macros to streamline your development process and maintain high-quality code.

