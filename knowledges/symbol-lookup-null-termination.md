# Dynamic Library Symbol Resolution & Null-Termination

## Summary
When loading shared libraries (`.so`, `.dll`, `.dylib`) at runtime in Rust using `libloading`, symbol names passed as byte slices should end with a null terminator byte (`\0`), such as `b"add\0"`.

## 1. Operating System C-ABI Requirements
Dynamic linkers at the operating system level are exposed via C APIs:
- **POSIX (Linux, macOS):** `dlsym(void *handle, const char *symbol)`
- **Windows:** `GetProcAddress(HMODULE hModule, LPCSTR lpProcName)`

Both APIs expect a C-style null-terminated string (`const char*`). C strings do not store length metadata; dynamic linkers determine string termination when encountering the first `0x00` (`\0`) byte.

## 2. Zero-Cost Allocation in `libloading`
Rust's `&str` and `&[u8]` types are slice references (`pointer + length`) and do not include an implicit null byte.

`libloading::Library::get` accepts `symbol: &[u8]`:
- **With `\0` (e.g., `b"add\0"`):** `libloading` passes the slice's pointer directly to `dlsym` or `GetProcAddress`. This avoids memory allocation.
- **Without `\0` (e.g., `b"add"`):** `libloading` must allocate a temporary buffer on the heap (`CString`), copy the slice bytes, append `\0`, and then pass the pointer to the OS API.

## Recommendation
Always append `\0` to byte literal symbol names (`b"function_name\0"`) when looking up symbols via `libloading` to avoid unnecessary heap allocations.
