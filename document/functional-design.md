# Functional Design: Sophisticate Phrase

This document details the functional design of the "Sophisticate Phrase" application, focusing on what the system will do from a user's perspective and how its core components will interact to deliver these functionalities.

## 1. Functional Requirements

The following are the key functional requirements for the Sophisticate Phrase application:

### 1.1. User Management & Authentication
*   **User Registration**: New users can create an account.
*   **User Login**: Registered users can log in using their credentials.
*   **User Logout**: Logged-in users can end their session.
*   **Session Management**: The system maintains user login state and allows access to user-specific data.

### 1.2. Phrase List Management
*   **Phrase List Import**:
    *   Users can upload a CSV formatted phrase list file.
    *   The system parses the CSV file and saves the phrase data to the database. If a list with the same name already exists for the user, new phrases are appended to it.
    *   Metadata for the phrase list (name, target language, source language, owner) is stored.
*   **Phrase List Display**:
    *   Logged-in users can view a list of their owned phrase lists (My Page).
    *   Users can select a specific phrase list and view its content.

### 1.3. Phrase Learning & Display
*   **Phrase Display Options**:
    *   Display all phrases within a list (both target and source languages).
    *   Display only the target language text.
    *   Display only the source language (translation) text.
*   **Text-to-Speech (TTS)**:
    *   The system can read aloud the text of displayed phrases.

### 1.4. User Interface & Navigation
*   **Top Page**: The application's entry point, providing navigation to login and registration.
*   **Login Page**: Screen for users to enter login credentials.
*   **Registration Page**: Screen for users to enter new account information.
*   **My Page**: User's dashboard, displaying a list of their owned phrase lists.
*   **Import Page**: Screen for uploading phrase list CSV files.
*   **Phrase List Page**: Displays the content of a selected phrase list and provides learning functionalities.

## 2. Major Modules/Components

The application will be composed of the following logical modules, each with distinct responsibilities:

### 2.1. User Management Module
*   **Responsibility**: Handles user registration, authentication (login/logout), session management, and user information persistence.
*   **Related Functional Requirements**: User Registration, User Login, User Logout, Session Management.

### 2.2. Phrase List Management Module
*   **Responsibility**: Manages the import process of phrase lists, handles phrase list metadata (name, languages, owner), and persists phrase list and phrase data to the database. This module is responsible for data retrieval from the database for display.
*   **Related Functional Requirements**: Phrase List Import, Phrase List Display (list and detail data retrieval).

### 2.3. Phrase Learning & Display Module
*   **Responsibility**: Manages the presentation of phrase list content (all, target-only, source-only views) and provides text-to-speech functionality. This module takes data from the Phrase List Management Module and renders it for the user.
*   **Related Functional Requirements**: Phrase Display Options, Text-to-Speech.

### 2.4. UI/Routing Module
*   **Responsibility**: Renders all application screens (Top Page, Login, Register, My Page, Import, Phrase List), handles routing based on user requests, and processes user interactions.
*   **Related Functional Requirements**: User Interface & Navigation (all aspects).

## 3. API Endpoints

The following API endpoints will support the defined functional requirements:

### 3.1. User Management & Authentication

*   **User Registration**
    *   **Method**: `POST`
    *   **Path**: `/register`
    *   **Description**: Creates a new user account.
    *   **Request Body**:
        ```json
        {
          "username": "string", // Required
          "password": "string"  // Required
        }
        ```
    *   **Response**:
        *   `200 OK`: Registration successful
            ```json
            {
              "message": "User registered successfully."
            }
            ```
        *   `400 Bad Request`: Invalid input, username already exists, etc.
            ```json
            {
              "error": "Username already exists."
            }
            ```

*   **User Login**
    *   **Method**: `POST`
    *   **Path**: `/login`
    *   **Description**: Authenticates a user and starts a session.
    *   **Request Body**:
        ```json
        {
          "username": "string", // Required
          "password": "string"  // Required
        }
        ```
    *   **Response**:
        *   `200 OK`: Login successful (sets session cookie)
            ```json
            {
              "message": "Login successful."
            }
            ```
        *   `401 Unauthorized`: Authentication failed (invalid username or password)
            ```json
            {
              "error": "Invalid username or password."
            }
            ```

*   **User Logout**
    *   **Method**: `POST`
    *   **Path**: `/logout`
    *   **Description**: Ends the current session.
    *   **Request Body**: None
    *   **Response**:
        *   `200 OK`: Logout successful (clears session cookie)
            ```json
            {
              "message": "Logout successful."
            }
            ```

### 3.2. Phrase List Management

*   **Phrase List Import Page Display**
    *   **Method**: `GET`
    *   **Path**: `/import`
    *   **Description**: Displays the form page for importing phrase lists.
    *   **Request Parameters**: None
    *   **Response**: HTML page

*   **Phrase List Import Processing**
    *   **Method**: `POST`
    *   **Path**: `/import`
    *   **Description**: Uploads a CSV formatted phrase list file and saves it to the database. If a list with the same name exists for the user, new phrases are appended to it.
    *   **Request Body**: `multipart/form-data` (file upload)
        *   `file`: CSV file (Required)
        *   `list_name`: string (Required) - Name of the list to import
        *   `target_lang`: string (Required) - Target language code (e.g., "en")
        *   `source_lang`: string (Required) - Source language code (e.g., "ja")
    *   **Response**:
        *   `302 Found`: Redirects to My Page after successful import
        *   `400 Bad Request`: Invalid file format, missing parameters, duplicate list name, etc.
            ```json
            {
              "error": "Invalid file format or missing parameters."
            }
            ```
        *   `401 Unauthorized`: Unauthenticated user

*   **User's Phrase List Display (My Page)**
    *   **Method**: `GET`
    *   **Path**: `/mypage`
    *   **Description**: Displays a list of phrase lists owned by the logged-in user.
    *   **Request Parameters**: None
    *   **Response**: HTML page (containing phrase list metadata)

### 3.3. Phrase Learning & Display

*   **Specific Phrase List Display (by ID)**
    *   **Method**: `GET`
    *   **Path**: `/list/{list_id}`
    *   **Description**: Displays the content of the phrase list specified by ID.
    *   **Path Parameters**:
        *   `list_id`: integer (Required) - ID of the phrase list to display
    *   **Query Parameters**:
        *   `display_mode`: string (Optional) - Display mode ("all", "target_only", "source_only"). Default is "all".
    *   **Response**: HTML page (containing phrase data)
        *   `404 Not Found`: Specified list ID does not exist or access denied.

*   **Specific Phrase List Display (by Name)**
    *   **Method**: `GET`
    *   **Path**: `/list/by_name`
    *   **Description**: Displays the content of a phrase list specified by name, owned by the current user.
    *   **Query Parameters**:
        *   `name`: string (Required) - Name of the phrase list to display
        *   `display_mode`: string (Optional) - Display mode ("all", "target_only", "source_only"). Default is "all".
    *   **Response**:
        *   `302 Found`: Redirects to the corresponding `GET /list/{list_id}` if the named list is found.
        *   `404 Not Found`: Specified named list does not exist or access denied.

*   **Text-to-Speech**
    *   **Method**: `GET`
    *   **Path**: `/speech`
    *   **Description**: Provides audio for the specified text.
    *   **Query Parameters**:
        *   `text`: string (Required) - Text to be spoken
        *   `lang`: string (Optional) - Language code of the text (e.g., "en", "ja"). Default is system default.
    *   **Response**: Audio data (e.g., `audio/mpeg` or `audio/wav`)
        *   `400 Bad Request`: Missing `text` parameter, etc.

### 3.4. Other

*   **Top Page Display**
    *   **Method**: `GET`
    *   **Path**: `/`
    *   **Description**: Displays the application's top page. Redirects to My Page if logged in.
    *   **Request Parameters**: None
    *   **Response**: HTML page

## 4. Data Model

The database schema will consist of three main tables: `users`, `phrase_lists`, and `phrases`.

```mermaid
erDiagram
    USERS ||--o{ PHRASE_LISTS : "owns"

    USERS {
        int id PK "User ID"
        string username "Username" UNIQUE
        string password_hash "Hashed Password"
        datetime created_at "Timestamp of creation"
    }

    PHRASE_LISTS ||--|{ PHRASES : "contains"

    PHRASE_LISTS {
        int id PK "List ID"
        int user_id FK "Owner of the list"
        string name "Name of the list (e.g., from filename)" UNIQUE_PER_USER_POSTGRES
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

### 4.1. Table Details

*   **`USERS` Table**: Stores user authentication information.
    *   `id`: Primary Key, unique user identifier.
    *   `username`: Unique identifier for the user, used for login. **(UNIQUE constraint)**
    *   `password_hash`: Hashed password for security.
    *   `created_at`: Timestamp of user creation.

*   **`PHRASE_LISTS` Table**: Stores metadata for each phrase list.
    *   `id`: Primary Key, unique list identifier.
    *   `user_id`: Foreign Key referencing `USERS.id`, indicating the owner of the list.
    *   `name`: Name of the phrase list. If a list with the same name exists for the user, new phrases are appended to it. **(UNIQUE constraint combined with `user_id`)**
    *   `target_lang`: Language code for the target language (e.g., "en").
    *   `source_lang`: Language code for the source language (e.g., "ja").
    *   `created_at`: Timestamp of list import.

*   **`PHRASES` Table**: Stores individual phrase entries.
    *   `id`: Primary Key, unique phrase identifier.
    *   `list_id`: Foreign Key referencing `PHRASE_LISTS.id`, indicating which list the phrase belongs to.
    *   `target_lang_text`: The phrase text in the target language.
    *   `source_lang_text`: The translation of the phrase in the source language.
