# messages-desktop

![Tauri](https://img.shields.io/badge/Tauri-24C8D8.svg?style=for-the-badge&logo=Tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-CE422B.svg?style=for-the-badge&logo=Rust&logoColor=white)
![macOS](https://img.shields.io/badge/macOS-000000.svg?style=for-the-badge&logo=Apple&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D4.svg?style=for-the-badge&logo=Windows&logoColor=white)
![Task](https://img.shields.io/badge/Task-29BEB0.svg?style=for-the-badge&logo=Task&logoColor=white)

Google Messages desktop app for macOS, powered by Tauri and WebKit.

## What it does

This app wraps [messages.google.com/web](https://messages.google.com/web) in a native macOS window. It lets you send and receive SMS and RCS messages from your Mac, using your Android phone as the connection point.

There is very little code in this project. The app opens a WebView that loads the official Google Messages website. No data passes through any third-party server.

## Google sign-in trial

This branch tests Google account pairing inside Tauri. On macOS, the WebView uses a Safari user agent that matches its WebKit engine. It does not change `navigator` properties or create a fake `window.chrome` object.

Google sign-in popups stay in the app. They use the opener's browser configuration so the login window and Messages share the same session. Other web links opened in a new window go to the default browser. Remote pages have no access to Tauri commands.

1. Open Messages and choose **Sign in with Google**.
2. Use the same Google account as the Messages app on your Android phone.
3. Complete any password and two-step verification prompts in the app.
4. Confirm the pairing on your phone by selecting the matching emoji.
5. Close and reopen the app to check that the pairing is saved.

Google is retiring QR code pairing. Google account sign-in in embedded browsers can still be rejected; this is an experiment, not a confirmed fix. Local verification loaded conversations in Tauri with an existing Google session. A fresh sign-in still needs to be tested. A successful login in Chrome does not sign in the Tauri WebView.

The trial is based on the [Android Messages Desktop login fix](https://github.com/OrangeDrangon/android-messages-desktop/commit/fbd8baff598e997a2a401897f7adecedba28b329) and [Tauri's popup handling](https://github.com/tauri-apps/tauri/issues/14263#issuecomment-3384966182). Android Messages Desktop uses Electron, so its result does not guarantee success with WebKit.

## Build

Requirements: [Rust](https://rustup.rs), [Node.js](https://nodejs.org), [Task](https://taskfile.dev).

```bash
task test      # checks popup routing
task lint      # checks Rust formatting and warnings
task package   # builds the .dmg
```

---

## macOS: "Messages is damaged and can't be opened"

This warning comes from macOS Gatekeeper, not from the app itself. Gatekeeper blocks any application that is not signed with an Apple Developer certificate ($99/year). When you download the .dmg from the internet, your browser adds a quarantine flag to the file, and macOS refuses to open it.

The app works perfectly fine — it is just not code-signed.

To fix it, open a terminal and run:

```bash
xattr -cr /Applications/Messages.app
```

Then open the app normally. You only need to do this once.
