# zeroize_alloc
This crate zeros all memory before freeing it, so if you keep secrets on the heap, you can be sure they no longer exist once they are freed.

## Usage
```rust
#[global_allocator]
static ALLOC: zeroize_alloc::ZeroizingAlloc<YourAllocator> = ZeroizingAlloc(YourAllocator);
```

If you want to use the default allocator, use `std::alloc::System`.