---
description: Verify frontend and backend builds successfully
---

Check that both the frontend and backend build successfully.

Steps:
1. Build frontend:
   - Run: `npm run build`
   - Check for TypeScript errors
   - Check for build warnings
2. Build backend:
   - Run: `cargo build --manifest-path=./src-tauri/Cargo.toml`
   - Check for Rust errors
   - Check for Clippy warnings with: `cargo clippy --manifest-path=./src-tauri/Cargo.toml`
3. Check formatting:
   - Frontend: Report if there's a lint command
   - Backend: `cargo fmt --manifest-path=./src-tauri/Cargo.toml --check`
4. Report summary:
   - ✓ Frontend build: Success/Failed
   - ✓ Backend build: Success/Failed
   - ✓ Linting: Success/Failed
   - List any errors or warnings

If builds fail, suggest fixes based on error messages.
