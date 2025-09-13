# Rilevazioni Sopralluogo App

## About The Project

This desktop application is designed to simplify and streamline the process of data collection during on-site surveys (`sopraluoghi`). It provides a user-friendly interface for both on-field and office data entry, backed by a robust Rust backend.

This is a desktop application designed to simplify and streamline the process of data collection during on-site surveys (`sopralluoghi`). It provides a user-friendly interface for technicians to input and manage survey data efficiently, both in the field and in the office.

## Core Functionality

The application operates by processing an initial `.xlsx` file containing survey data. It imports this information into a local SQLite database, creating a dedicated file for each case (`fascicolo`) within the user's `Documents` directory. From that point on, the application uses the SQLite database as the source of truth, allowing for modifications and additions. The updated data can then be exported back into a new `.xlsx` file.

## Key Features

*   **Dashboard Overview:** A comprehensive dashboard (`panoramica`) to visualize key metrics of the selected building, including fixture counts, material distribution, and room completion status.
*   **Data Entry & Management:**
    *   Intuitive forms for adding new fixtures (`infissi`), specifying details like dimensions, type, material, and glass.
    *   Modify room details such as height, wall thickness, heating/cooling systems, and lighting.
    *   Attach annotations and notes to buildings, rooms, and fixtures.
    *   Manage building utility information (`utenze`) and photovoltaic system data.
*   **Data Tables:**
    *   Interactive tables for viewing all rooms and fixtures.
    *   In-place editing of table data.
    *   Advanced sorting and filtering capabilities for all data grids.
*   **File Management:**
    *   Import data from an `.xlsx` file to initialize a new survey project.
    *   Seamlessly switch between different survey databases (`fascicoli`).
    *   Export all collected and modified data back to an `.xlsx` file.
*   **User Interface:**
    *   A clean, modern UI built with shadcn/ui.
    *   Support for both Light and Dark themes.
    *   Responsive layout with a collapsible sidebar.

## Architecture

The application is built using the **Tauri** framework, which combines a modern web frontend with a performant Rust backend.

*   **Frontend (`src/`)**: A **React** application built with Vite and styled using Tailwind CSS.
    *   **UI Components**: A rich set of reusable components built upon `shadcn/ui` for forms, tables, dialogs, and navigation.
    *   **State Management**: React's Context API is used extensively to provide global state for buildings, fixtures, rooms, and other application data.
    *   **Routing**: `react-router-dom` handles navigation between the main pages of the application (Dashboard, Data Entry, and Overview).

*   **Backend (`src-tauri/`)**: A **Rust** application that manages the core logic and data persistence.
    *   **API**: The backend exposes commands to the frontend for all data operations (CRUD).
    *   **Database**: Uses `rusqlite` to interact with a local SQLite database. It includes a custom query builder for constructing dynamic SQL statements.
    *   **File Handling**: Leverages `calamine` for importing `.xlsx` files and `rust_xlsxwriter` for exporting data back to Excel. The backend is responsible for creating and managing the lifecycle of the SQLite database files.

## Built With

This application is built using the Tauri framework, combining a modern web frontend with a powerful Rust backend.

*   **Backend**: [Rust](https://www.rust-lang.org/) with [Tauri](https://tauri.app/)
*   **Frontend**: [React](https://reactjs.org/), [TypeScript](https://www.typescriptlang.org/)
*   **UI Components**: [shadcn/ui](https://ui.shadcn.com/)
*   **Database**: [SQLite](https://www.sqlite.org/index.html)

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

*   Node.js & npm
*   Rust & Cargo
*   Tauri

### Installation & Running

1.  Clone the repository:
    ```sh
    git clone https://github.com/Sir-Vlad/rilevazioni_sopraluogo_app.git
    cd rilevazioni_sopraluogo_app
    ```
2.  Install NPM packages:
    ```sh
    npm install
    ```
3.  Run the application in development mode:
    ```sh
    npm run tauri:dev
    ```

### Building the Application

To build a distributable binary for your platform, run:

```sh
npm run tauri:build
```

The output will be located in the `src-tauri/target/release/bundle/` directory.

## Project Structure

The project is organized into two main parts: the frontend and the backend.

*   `src/`: Contains the React/TypeScript frontend application.
    *   `pages/`: Main application views (Dashboard, Data Entry, Overview).
    *   `components/`: Reusable React components, including UI elements built with shadcn/ui.
    *   `context/`: React Context providers for managing global state (e.g., database connection, building data).
    *   `hooks/`: Custom React hooks for shared logic.
*   `src-tauri/`: Contains the Rust backend logic.
    *   `src/command.rs`: Defines the functions (`Tauri commands`) that can be invoked from the frontend.
    *   `src/service/`: High-level business logic, including data import/export and service layers for different data entities.
    *   `src/dao/`: Data Access Objects (DAOs) for direct interaction with the SQLite database.
    *   `src/database/`: Manages the database connection and includes a custom SQL query builder.
    *   `src/dto/`: Data Transfer Objects (DTOs) used for communication between the frontend and backend.
    *   `tauri.conf.json`: The main Tauri configuration file for the application.
