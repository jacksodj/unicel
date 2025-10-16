---
description: Verify frontend and backend build successfully
---

Check that both frontend and backend build successfully.

I will:
1. **Build frontend**: Run `npm run build` and check for TypeScript errors
2. **Build backend**: Run `cargo build` and check for Rust errors
3. **Run Clippy**: Check for Rust warnings with `cargo clippy`
4. **Check formatting**: Verify code formatting with `cargo fmt --check`

Report summary:
- ✓ Frontend build: Success/Failed
- ✓ Backend build: Success/Failed
- ✓ Clippy: Success/Failed
- ✓ Formatting: Success/Failed

If builds fail, I'll suggest fixes based on error messages.
