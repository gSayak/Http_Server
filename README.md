[![progress-banner](https://backend.codecrafters.io/progress/http-server/d6b5d33e-5a2c-4fb0-8812-e2f1cde0a3e8)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

# Multi-Client HTTP/1.1 Server

## Overview

This project is a multi-threaded HTTP/1.1 server that serves clients over TCP. It supports various features, including handling GET and POST requests, parsing headers, and responding with appropriate status codes.

## Getting Started

1. **Start the Server**: Begin by launching the server on port 4221.

    ```bash
    ./your_server.sh
    ```

2. **Accepting Connections**: The server accepts TCP connections and responds with a 200 OK status.

3. **Handling Errors**: In case of an error, the server gracefully responds with a 404 NOT FOUND status.

## Implemented Features

### 1. GET Request Handling

The server effectively processes GET requests and serves content accordingly.

### 2. Header Parsing

Parsed headers, extracting and printing the `user-agent` information.

### 3. Multi-Threading

Implemented multi-threading to handle multiple clients concurrently.

### 4. Directory Viewing

Utilize GET requests with the `--directory <directory-path>` argument to view the contents of a specified directory.

### 5. File Upload via POST

Leverage POST requests to save files in the directory of the web server. The server responds with a 201 OK status upon successful file creation.


