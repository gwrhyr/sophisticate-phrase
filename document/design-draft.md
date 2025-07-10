# Design Draft: Sophisticate Phrase

This document outlines the design draft for the "Sophisticate Phrase" application based on the requirements.

## 1. Overview

Sophisticate Phrase is a web-based language learning application designed to help users learn new languages through phrases rather than individual words. Users can manage their own custom phrase lists and utilize various learning modes.

## 2. Architecture

The application will be a monolithic web application built with Rust.

- **Backend:** Rust with a web framework like `axum` or `actix-web`. It will handle user authentication, phrase list management, and rendering HTML templates.
- **Frontend:** Server-side rendered HTML with minimal JavaScript for interactive features like text-to-speech.
- **Database:** PostgreSQL for robust data storage and scalability.

```mermaid
graph TD
    A[User] --> B{Browser}
    B --> C[Rust Web Server]
    C --> D[Database]
    C --> B
```

### 2.1. Deployment Strategy

To prioritize cost-effectiveness and portability, the application will be deployed using Docker containers on a cost-efficient IaaS provider.

-   **IaaS Provider**: A Virtual Private Server (VPS) provider such as DigitalOcean, Linode, or Vultr will be utilized. These providers offer competitive pricing and straightforward virtual machine instances suitable for this application's scale.
-   **Containerization**: The application will be containerized using Docker. This ensures high portability across different environments (development, staging, production) and IaaS providers, minimizing vendor lock-in and simplifying deployment.
-   **Orchestration**: For single-instance deployments, Docker Compose will be used to manage the application container. For future scaling or more complex setups, Kubernetes could be considered, but it is out of scope for the initial design.
-   **Reverse Proxy**: An Nginx reverse proxy will be used in front of the Docker container to handle SSL/TLS termination, static file serving, and request forwarding to the application. This acts as a 'gatekeeper' for the application server, enhancing security, improving performance by offloading static content delivery, and enabling flexible request routing.
-   **Provisioning**: For server provisioning and configuration management, tools like Ansible will be considered to automate Docker installation and application deployment.

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

    User->>Browser: Select CSV file and submit
Browser->>Server: POST /import (multipart/form-data)
Server->>Server: Parse CSV file
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
| POST   | /import               | Handles the import of a CSV phrase list. If a list with the same name exists for the user, new phrases are added to it. |
| GET    | /list/{list_id}       | Displays a specific phrase list. Supports `display_mode` query parameter (all, target_only, source_only). |
| GET    | /speech               | Browser-based Text-to-Speech functionality (Web Speech API). |

## 7. Open Issues / Considerations

- **Text-to-Speech (TTS) Implementation:** Browser-based Web Speech API is currently implemented. Future consideration for server-side integration with a cloud service for higher quality.
- **CSV Parsing:** Define strict rules for the CSV format, especially for language keys (e.g., `en`, `ja`). The first entry can be used to determine the languages for the entire list.
- **Error Handling:** Detailed error handling for file uploads (wrong format, too large) and database operations.
- **Scalability:** PostgreSQL is already in use for robust data storage and scalability.
