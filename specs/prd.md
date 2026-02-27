# Product Requirements Document

## OpenPrompts — Windows Desktop Prompt Manager

## 1. Problem Statement

Power users of AI tools (ChatGPT, Claude, Copilot, etc.) repeatedly type or hunt for the same prompts across sessions and applications. There is no fast, friction-free way to store a personal library of prompts and inject them exactly where needed — without context-switching away from work.

---

## 2. Product Vision

A lightweight Windows desktop app that lives quietly in the system tray, giving users instant access to their personal prompt library from anywhere on the desktop — any browser, any app, any text field — with a single hotkey.

---

## 3. Goals

- **Reduce friction** between having a good prompt and using it.
- **Centralize** prompt storage so nothing is lost in browser tabs, Notion pages, or sticky notes.
- **Stay out of the way** — the app should feel invisible until needed.

---

## 4. Target Users

- AI power users who rely on curated prompts for writing, coding, research, or support.
- Professionals who use AI tools across multiple applications daily.
- Teams that want to share and standardize prompts (future scope).

---

## 5. Core Features

### 5.1 Prompt Library
- Store prompts with a title, body, and optional folder/category.
- Full CRUD: create, edit, duplicate, delete.
- Organize via folders or tags to support a growing library.

### 5.2 Global Search
- Invoke a quick-search overlay from anywhere on the desktop via a global hotkey.
- Fuzzy search across prompt titles and body text.
- Results ranked by recency or relevance.

### 5.3 Instant Paste
- Selecting a prompt from the overlay pastes it directly into the previously focused application — no manual copy-paste step.
- Works in any app: browsers, IDEs, terminals, Office apps.

### 5.4 System Tray Presence
- App runs as a background process accessible from the system tray.
- Minimal resource footprint; does not interrupt foreground work.

### 5.5 Quick-Add
- Capture new prompts from a lightweight input without opening the full app window.

---

## 6. Non-Goals (v1)

- Cloud sync or cross-device access.
- Team collaboration or prompt sharing.
- Native macOS or Linux support.
- AI-generated prompt suggestions.
- Browser extension.

---

## 7. Key User Flows

**Flow A — Use a saved prompt:**
1. User is focused in any app (e.g., ChatGPT in Chrome).
2. User presses global hotkey → overlay appears.
3. User types a keyword → matching prompts surface instantly.
4. User selects a prompt → overlay closes → prompt text is pasted into the focused field.

**Flow B — Save a new prompt:**
1. User opens tray icon or quick-add hotkey.
2. User enters title + prompt body, assigns a folder.
3. Prompt is saved and immediately searchable.

---

## 8. Success Metrics

- Time from "need a prompt" to "prompt pasted" < 5 seconds.
- Users maintain a library of 20+ prompts with no degradation in search speed.
- App launches on startup and stays resident without noticeable memory impact.

---

## 9. Constraints & Assumptions

- Windows 11 only (v1).
- Local storage only; data stays on device.
- Paste mechanism uses the system clipboard temporarily (with immediate restoration where feasible).
- Hotkey must be configurable to avoid conflicts with other tools.

---

## 10. Open Questions

- Should the overlay support variable placeholders in prompts (e.g., `{{topic}}`)?
- What is the seeded/default prompt library on first launch?
- Is import/export (JSON, CSV) in scope for v1?
- Would it be worth trying Tauri as the desktop framework to keep the app lightweight?
- Framework decision (compressed): Tauri (smaller footprint, aligns with current stack) vs .NET MAUI (C#/.NET-first, larger replatform effort) — default to Tauri unless full .NET alignment is a priority.
