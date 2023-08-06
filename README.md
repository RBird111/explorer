# File Explorer

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Svelte](https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white)
![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF)
![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)

![App Image](./static/app-preview.png)

## What does it do?

It's an app that lets you navigate your computer's file system...and that's pretty much it for now. You can't interact with files in any meaningful way.

## How to install?

This was just a quick app meant to help me get familiar with [Tauri](https://tauri.app/) so I'm not going to bother linking any precompiled binaries but you can compile it yourself in a few quick steps.

1. First you're going to need some kind of node package manager (npm, yarn, pnpm, etc.). I'm guessing this step is already done for most people looking at this page.

2. In order to compile the app you're going to need to install Rust. Just follow the instructions [here](https://www.rust-lang.org/learn/get-started).

3. Clone the repo. Just run `git clone https://github.com/RBird111/explorer`

4. Install dependencies.

```bash
cd explorer
npm install
```

5. Run it.

- Developer mode:

```bash
npm tauri dev
```

- Production build:

```bash
npm tauri build
./src-tauri/target/release/file-view
```

## What's it made of?

### [Rust](https://www.rust-lang.org/)

### [Svelte](https://kit.svelte.dev/)

### [Tauri](https://tauri.app/)
