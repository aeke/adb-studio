# 🤝 Contributing to ADB Studio

Thank you for your interest in contributing to **[ADB Studio](https://github.com/aeke/adb-studio)**!  
We welcome all contributions — from bug reports and feature requests to code improvements and documentation updates.

---

## 🧭 Table of Contents
- [Code of Conduct](#-code-of-conduct)
- [How to Contribute](#-how-to-contribute)
- [Development Setup](#-development-setup)
- [Pull Request Guidelines](#-pull-request-guidelines)
- [Feature Requests](#-feature-requests)
- [Reporting Bugs](#-reporting-bugs)
- [Community & Support](#-community--support)

---

## 📜 Code of Conduct
Please be respectful, open-minded, and constructive.  
We follow the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct.html) for all interactions.

---

## 🛠️ How to Contribute

### 1. Fork & Clone
Fork the repository and clone your fork locally:

```bash
git clone https://github.com/<your-username>/adb-studio.git
cd adb-studio
```

### 2. Create a Branch
Use descriptive branch names for your work:
```bash
git checkout -b fix/device-detection-bug
# or
git checkout -b feature/apk-batch-install
```

### 3. Make Changes
- Follow the existing code style and architecture.
- Add comments for complex logic.
- Test your code on at least one platform (macOS, Windows, or Linux).

### 4. Commit & Push
Write clear, conventional commit messages:

```bash
git add .
git commit -m "fix: handle adb device detection timeout"
git push origin fix/device-detection-bug
```

### 5. Open a Pull Request
Go to your fork → click **“Compare & Pull Request”** → describe your changes clearly.

---

## 🧩 Development Setup

### Requirements
- **Rust** 1.70+  
- **Cargo** package manager  
- **ADB** installed and available in your system PATH  
- Android device with **USB debugging enabled**

### Run in development mode
```bash
cargo run
```

### Build release version
```bash
cargo build --release
```

### Run formatter and linter
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 🚦 Pull Request Guidelines
✅ Keep pull requests small and focused.  
✅ Link related issues using `Fixes #123`.  
✅ Ensure code compiles on all major platforms.  
✅ Include screenshots for UI-related changes.  
✅ Update documentation or README if relevant.  

---

## 💡 Feature Requests
Have an idea for a new feature?  
Open an issue with the label **`enhancement`** and describe:
- The problem you want to solve
- Your proposed solution
- Mockups, screenshots, or references (if available)

👉 [Create a feature request here](https://github.com/aeke/adb-studio/issues/new?labels=enhancement&template=feature_request.md)

---

## 🐞 Reporting Bugs
If you encounter a bug:
1. Check [existing issues](https://github.com/aeke/adb-studio/issues)
2. If not found, open a **new issue**
3. Include:
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots or logs (if possible)

👉 [Report a bug here](https://github.com/aeke/adb-studio/issues/new?labels=bug&template=bug_report.md)

---

## 💬 Community & Support
Join the discussion on [GitHub Discussions](https://github.com/aeke/adb-studio/discussions)  
We appreciate your feedback, suggestions, and contributions!

---

> 🧡 Thank you for helping make **ADB Studio** better!
