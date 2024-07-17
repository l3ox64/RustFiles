# File Manager over HTTP with Rust

This project is a simple web application built in Rust using the Axum framework, allowing users to share files and folders over HTTP. It provides functionalities to browse directories and download files. Consider using this web app for simple online file sharing.

## How to Use

1. **Setup:**
   - Clone the repository: `git clone <repository-url>`
   - Install Rust and dependencies: `cargo build`

2. **Run:**
   - Start the server: `cargo run`
   - Access the application at [http://localhost:3000](http://localhost:3000)

3. **Usage:**
   - Upload files inside the /files directory on the server via CLI
   - Navigate through directories by clicking on folder names.
   - Download files by clicking on their names or using the “Download” button.
   - Download entire folders by clicking the “Download Folder” button.

## Technologies Used

- **Rust:** Programming language used to build the backend.
- **Axum:** Rust web framework for building asynchronous HTTP applications.
