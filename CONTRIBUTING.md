# Contributing to VoltChain ⚡

Thank you for your interest in contributing to VoltChain! We are building a decentralized future for community energy, and we welcome developers, designers, and enthusiasts.

## 🌈 Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct (keep it professional, respectful, and collaborative).

## 🛠️ Development Workflow

### 1. Fork and Clone
- Fork the repository on GitHub.
- Clone your fork locally:
  ```bash
  git clone git@github.com:your-username/voltchain.git
  cd voltchain
  ```

### 2. Branching Strategy
- Create a feature branch from `main`:
  ```bash
  git checkout -b feat/your-feature-name
  ```
- Use the following prefixes for branches:
  - `feat/`: New features
  - `fix/`: Bug fixes
  - `docs/`: Documentation updates
  - `refactor/`: Code improvements without behavior changes
  - `test/`: Adding or updating tests

### 3. Coding Standards

#### Rust (Backend & Contracts)
- **Formatting**: Always run `cargo fmt` before committing.
- **Linting**: Use `cargo clippy` to ensure high-quality code.
- **Testing**: Ensure all tests pass with `cargo test`.
- **Documentation**: Use `///` for public methods and structs.

#### Frontend (Next.js & TypeScript)
- **Type Safety**: Avoid using `any`. Define interfaces for all data models.
- **Components**: Use functional components and hooks.
- **Styling**: Adhere to the existing TailwindCSS design system.

### 4. Commit Messages
We follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat: add freighter wallet support`
- `fix: resolve db connection leak`
- `docs: update setup instructions`

---

## 🧪 Testing Requirements

Every Pull Request must include relevant tests:
- **Contracts**: Rust unit tests in `src/test.rs`.
- **Backend**: Integration tests for new API endpoints.
- **Frontend**: Component tests if applicable.

---

## 🚀 Submission Process

1.  **Sync your fork**: `git pull upstream main`.
2.  **Push your branch**: `git push origin feat/your-feature-name`.
3.  **Open a Pull Request**: Provide a clear description of the changes and link any related issues.
4.  **Code Review**: At least one maintainer must approve the PR before merging.

## 💎 Rewards & Recognition
Contributors to VoltChain are recognized in our `CONTRIBUTORS.md` and may be eligible for community bounties in the future!

---

**Questions?** Reach out to us via [Discord](https://discord.gg/voltchain) or open an issue on GitHub.
