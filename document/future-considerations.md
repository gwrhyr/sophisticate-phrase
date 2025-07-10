# Future Considerations and Potential Challenges

This document outlines potential challenges and areas for future consideration that may arise as the Sophisticate Phrase application evolves.

## 1. Mobile Native App Integration with Server-Side Rendering (SSR)

**Current State:**
The current architecture primarily relies on Server-Side Rendering (SSR) for the web frontend. The Rust backend generates and serves HTML directly to the browser, with minimal JavaScript for interactive elements.

**Potential Challenge:**
If there is a future requirement to develop mobile native applications (iOS/Android), the current SSR-heavy approach presents challenges for seamless integration:

-   **Lack of Dedicated APIs**: The backend primarily serves HTML, meaning explicit RESTful or GraphQL APIs providing raw data (e.g., JSON) for native apps are not inherently available.
-   **Logic Duplication/Tight Coupling**: UI rendering logic and some business logic are tightly coupled within the backend's HTML generation. This could lead to duplication of logic if re-implemented on the mobile side, or require significant refactoring to expose these as reusable services via APIs.
-   **Increased Development Effort for Mobile**: Developing native apps would necessitate building a separate UI layer and potentially a new API layer on the backend, leading to a larger development effort compared to a unified API-driven approach from the start.

**Future Evolution Strategy (if needed):**
Should mobile native app integration become a priority, the architecture would need to evolve:

-   **Introduce a Dedicated API Layer**: Implement explicit RESTful or GraphQL APIs in the backend to serve structured data to mobile applications (and potentially to a refactored web frontend).
-   **Decouple Frontend Logic**: Re-evaluate and refactor backend logic to separate data provision from UI rendering concerns.
-   **Web Frontend Adaptation**: The existing web frontend might transition to a more API-driven approach (e.g., using a JavaScript framework for client-side rendering) to consume the same APIs as the mobile apps, promoting consistency and reducing redundant backend logic.

## 2. Phrase List Data Format Evolution

**Current State:**
The initial design utilizes CSV (Comma Separated Values) for phrase list imports, prioritizing user-friendliness for manual creation and editing due to its simplicity and widespread tool support (e.g., spreadsheets).

**Future Consideration:**
While CSV is excellent for simple key-value pairs, its limitations become apparent with more complex data structures. Future enhancements might require a more robust format:

-   **JSON (JavaScript Object Notation)**: Offers strong structural capabilities, widely supported by programming languages and web APIs. It's less verbose than XML and generally easier for machines to parse.
-   **YAML (YAML Ain't Markup Language)**: Provides similar structural capabilities to JSON but with a focus on human readability, often used for configuration files. However, its strict indentation rules can be a source of error for manual editing.

The choice between JSON and YAML for future iterations would depend on the balance between human readability (YAML) and machine processability/tooling ecosystem (JSON), especially if phrase lists evolve to include more metadata (e.g., difficulty, context, audio file references, example sentences).

## 3. Session Management Evolution

**Current State:**
For initial development and simplicity, a basic cookie-based session management system is implemented. Session IDs are stored in HTTP-only cookies, and session data (mapping session ID to user ID) is stored in an in-memory HashMap on the server. The recent fix addressed correct cookie handling, but the underlying in-memory session store remains the same.

**Future Evolution Strategy:**
While the current in-memory session store is sufficient for development, it is not scalable for multi-instance deployments and has limitations regarding persistence across application restarts. For a production-ready application, the session management system should evolve to use a more robust and scalable solution.

-   **Adopt a Rust Session Management Crate**: Integrate a well-vetted Rust crate designed for secure and scalable session management (e.g., `tower-sessions` or similar). These crates typically provide features for secure session ID generation, cookie management, and integration with various session stores.
-   **Utilize a Distributed Session Store**: Transition from an in-memory HashMap to a distributed session store like Redis or a dedicated database table. This will enable horizontal scaling of the application by allowing multiple application instances to share session data, and provide persistence across application restarts.
