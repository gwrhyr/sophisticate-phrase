# Design Draft: Sophisticate Phrase

This document outlines the design draft for the "Sophisticate Phrase" application based on the requirements.

## 1. Overview

Sophisticate Phrase is a web-based language learning application designed to help users learn new languages through phrases rather than individual words. Users can manage their own custom phrase lists and utilize various learning modes.

## 2. Architecture

The application will be a monolithic web application built with Rust.

- **Backend:** Rust with a web framework like `axum` or `actix-web`. It will handle user authentication, phrase list management, and rendering HTML templates.
- **Frontend:** Server-side rendered HTML with minimal JavaScript for interactive features like text-to-speech.
- **Database:** SQLite for simplicity to store user and phrase data.

```mermaid
graph TD
    A[User] --> B{Browser};
    B --> C[Rust Web Server];
    C --> D[Database (SQLite)];
    C --> B;
```

## 3. Data Model (ER Diagram)

The database will consist of three main tables: `users`, `phrase_lists`, and `phrases`.

```mermaid
erDiagram
    USERS ||--o{ PHRASE_LISTS : "owns"

    USERS {
        int id PK "User ID"
        string username "Username"
        string password_hash "Hashed Password"
        datetime created_at "Timestamp of creation"
    }

    PHRASE_LISTS ||--|{ PHRASES : "contains"

    PHRASE_LISTS {
        int id PK "List ID"
        int user_id FK "Owner of the list"
        string name "Name of the list (e.g., from filename)"
        string target_lang "e.g., en"
        string source_lang "e.g., ja"
        datetime created_at "Timestamp of import"
    }

    PHRASES {
        int id PK "Phrase ID"
        int list_id FK "Belongs to which list"
        string target_lang_text "Text in the target language"
        string source_lang_text "Text in the source language (translation)"
    }
```

## 4. Screen Transition Diagram

This diagram shows the flow of screens a user will navigate.

```mermaid
stateDiagram-v2
    [*] --> TopPage : Visit site
    TopPage --> LoginPage : Click Login
    TopPage --> RegisterPage : Click Register
    RegisterPage --> MyPage : Registration successful
    LoginPage --> MyPage : Login successful
    LoginPage --> TopPage : Login failed

    MyPage --> PhraseListPage : Select a phrase list
    MyPage --> ImportPage : Click "Import New List"
    ImportPage --> MyPage : Import successful

    PhraseListPage --> PhraseListPage : Toggle display (Target/Source/Both)
    PhraseListPage --> MyPage : Back to MyPage

    MyPage --> [*] : Logout
```

## 5. Sequence Diagrams

### 5.1. User Login

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant Server
    participant Database

    User->>Browser: Enter credentials and submit
    Browser->>Server: POST /login (username, password)
    Server->>Database: SELECT id, password_hash FROM users WHERE username = ?
    Database-->>Server: User record
    alt Authentication Success
        Server->>Server: Generate session token
        Server-->>Browser: Redirect to /mypage (with session cookie)
        Browser->>User: Display MyPage
    else Authentication Failure
        Server-->>Browser: Render LoginPage with error
        Browser->>User: Display error message
    end
```

### 5.2. Phrase List Import

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant Server
    participant Database

    User->>Browser: Select YAML file and submit
    Browser->>Server: POST /import (multipart/form-data)
    Server->>Server: Parse YAML file
    Server->>Database: BEGIN TRANSACTION
    Server->>Database: INSERT INTO phrase_lists (...)
    Database-->>Server: list_id
    loop For each phrase in YAML
        Server->>Database: INSERT INTO phrases (list_id, ...)
    end
    Server->>Database: COMMIT
    Server-->>Browser: Redirect to /mypage
    Browser->>User: Display MyPage with new list
```

## 6. API Endpoints (Conceptual)

| Method | Path                  | Description                               |
|--------|-----------------------|-------------------------------------------|
| GET    | /                     | Shows the top page / login page.          |
| POST   | /login                | Authenticates a user.                     |
| POST   | /logout               | Logs out the user.                        |
| GET    | /mypage               | User's main page, shows all phrase lists. |
| GET    | /import               | Shows the file import page.               |
| POST   | /import               | Handles the import of a YAML phrase list. |
| GET    | /list/{list_id}       | Displays a specific phrase list.          |
| GET    | /speech               | Endpoint for Text-to-Speech functionality.|

## 7. Open Issues / Considerations

- **Text-to-Speech (TTS) Implementation:** The specific method for TTS needs to be decided. It could be a browser-based API (Web Speech API) or a server-side integration with a cloud service. A browser-based approach is simpler for a start.
- **YAML Parsing:** Define strict rules for the YAML format, especially for language keys (e.g., `en`, `ja`). The first entry can be used to determine the languages for the entire list.
- **Error Handling:** Detailed error handling for file uploads (wrong format, too large) and database operations.
- **Scalability:** SQLite is for simplicity. If the service grows, migrating to a more robust database like PostgreSQL would be necessary.
