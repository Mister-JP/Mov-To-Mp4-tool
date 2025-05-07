# Mov-To-Mp4-Tool

**A robust, web-based utility built with Rust and Actix-web for efficient conversion of MOV video files to the MP4 format using FFmpeg.**

This tool provides a clean, user-friendly interface for uploading MOV files and converting them to the widely compatible MP4 format. It's designed with a focus on simplicity for the end-user and a solid, asynchronous backend for handling file operations and transcoding.

## Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Running the Application](#running-the-application)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [How It Works (Architecture)](#how-it-works-architecture)
  - [Frontend (Client-Side)](#frontend-client-side)
  - [Backend (Server-Side)](#backend-server-side)
  - [FFmpeg Integration](#ffmpeg-integration)
- [Error Handling](#error-handling)
- [Potential Enhancements & Future Work](#potential-enhancements--future-work)
- [A Note on Security](#a-note-on-security)

## Features

*   **User-Friendly Web Interface:** Clean and intuitive UI for easy file interaction.
*   **Flexible File Upload:** Supports both drag & drop and traditional file selection for MOV files.
*   **Client-Side Validation:** Initial check for `.mov` file extension on the frontend.
*   **File Information Display:** Shows the name and size of the selected file before conversion.
*   **Efficient Backend Conversion:** Leverages `ffmpeg` for reliable and high-quality video transcoding.
    *   Converts video to **H.264 (AVC)** codec.
    *   Converts audio to **AAC** codec.
*   **Asynchronous Operations:** Built with Actix-web, enabling non-blocking request handling.
*   **Unique Output Filenames:** Uses UUIDs to generate unique names for converted files, preventing collisions.
*   **Progress Indication:** Frontend displays a simulated progress bar during conversion. (Note: Actual FFmpeg progress streaming is not implemented).
*   **Clear Feedback:** Provides distinct success and error messages, including user-friendly interpretations of common FFmpeg issues.
*   **Direct Download:** Offers a direct download link for the converted MP4 file.
*   **Organized File Storage:** Manages uploaded and converted files in dedicated `uploads/` and `output/` directories (created at runtime).

## Tech Stack

*   **Backend:**
    *   **Language:** Rust (2021 Edition)
    *   **Web Framework:** Actix-web `4.3.1`
    *   **Template Engine:** Tera `1.19.0`
    *   **Asynchronous Runtime:** Tokio `1.28.2`
    *   **Multipart Form Processing:** `actix-multipart` `0.6.0`
    *   **Serialization/Deserialization:** `serde`, `serde_json`
    *   **UUID Generation:** `uuid` `1.3.3` (with `v4` feature)
    *   **Logging:** `log`, `env_logger`
*   **Frontend:**
    *   HTML5
    *   CSS3 (with CSS Variables)
    *   Vanilla JavaScript (ES6+)
*   **Core Conversion Engine:**
    *   FFmpeg (External command-line dependency)

## Prerequisites

Before running this application, ensure you have the following installed:

1.  **Rust Toolchain:** Install Rust via [rustup](https://rustup.rs/). The latest stable version is recommended.
2.  **FFmpeg:** FFmpeg must be installed on your system and accessible via the system's `PATH`. The application relies on `ffmpeg` for probing and converting video files.

## Getting Started

### Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/Mister-JP/Mov-To-Mp4-tool.git
    cd Mov-To-Mp4-tool
    ```

2.  **Build the project:**
    The project uses Cargo, Rust's package manager and build system.
    ```bash
    cargo build --release
    ```
    Using the `--release` flag enables optimizations for a production-ready build. For development, you can omit it for faster compilation.

### Running the Application

Once built, you can run the application using:

```bash
cargo run --release
# Or, if already built:
# ./target/release/mov2mp4
```

By default, the server will start on `http://127.0.0.1:8082`. You'll see a log message confirming this:
`INFO  mov2mp4 > Starting server at http://127.0.0.1:8082`

## Usage

1.  Open your web browser and navigate to `http://127.0.0.1:8082`.
2.  You will see the "MOV to MP4 Converter" interface.
3.  **Upload a File:**
    *   Drag and drop your `.mov` file onto the designated drop area.
    *   Alternatively, click the "Select File" button and choose a `.mov` file from your system.
4.  The selected file's name and size will be displayed.
5.  Click the "Convert to MP4" button.
6.  A progress bar will indicate the (simulated) conversion process.
7.  **Results:**
    *   **Success:** A "Conversion Successful!" message will appear, along with a "Download MP4" button. Click it to download your converted file.
    *   **Failure:** A "Conversion Failed" message will appear with details about the error. An "Try Again" button allows you to reset the interface.
8.  Uploaded files are temporarily stored in the `uploads/` directory and converted files in the `output/` directory within the project's root.

## Project Structure

```
Mov-To-Mp4-tool/
├── .gitignore           # Specifies intentionally untracked files
├── Cargo.lock           # Records exact versions of dependencies
├── Cargo.toml           # Project manifest and dependencies
├── static/              # Static assets (CSS, JavaScript)
│   ├── css/
│   │   └── styles.css   # Stylesheet for the web interface
│   └── js/
│       └── app.js       # Client-side JavaScript for UI interactions
├── src/                 # Source code
│   ├── main.rs          # Main application logic (Actix-web server, handlers)
│   └── templates/
│       └── index.html   # Tera template for the main HTML page
├── uploads/             # (Created at runtime) Directory for temporary MOV uploads
└── output/              # (Created at runtime) Directory for converted MP4 files
```

## How It Works (Architecture)

The application follows a client-server architecture with FFmpeg as the processing backbone.

### Frontend (Client-Side)

*   **`index.html` (rendered by Tera):** Provides the main structure of the web page. Variables like `app_name` and `app_description` are injected by the backend.
*   **`static/css/styles.css`:** Defines the visual appearance of the application.
*   **`static/js/app.js`:**
    *   Manages all client-side interactions: drag & drop, file selection, button clicks.
    *   Performs basic file type validation (checks for `.mov` extension).
    *   Uses the Fetch API to send the selected file (as `FormData`) to the `/convert` endpoint.
    *   Updates the UI to show file information, simulated progress, and final results (success/error messages, download link).

### Backend (Server-Side - `src/main.rs`)

The backend is an Actix-web application with several key components:

1.  **Server Initialization:**
    *   Sets up logging using `env_logger`.
    *   Initializes the Tera template engine, loading `index.html`.
    *   Starts an HTTP server listening on `127.0.0.1:8082`.

2.  **Routing:**
    *   `GET /`: Serves the main `index.html` page, rendered by Tera.
    *   `POST /convert`: The core conversion endpoint.
        1.  Receives the uploaded file using `actix-multipart`.
        2.  Ensures `uploads/` and `output/` directories exist, creating them if necessary.
        3.  Saves the uploaded MOV file to the `uploads/` directory with its original filename.
        4.  **File Probing:** Before full conversion, `ffmpeg -i <input_file>` is executed. The `stderr` of this command is checked for critical errors like "Invalid data found" or "does not contain any stream". This allows for early failure if the file is fundamentally corrupt or not a valid video.
        5.  If the probe is satisfactory, a unique output filename (UUID-based `.mp4`) is generated for the `output/` directory.
        6.  **Conversion:** Invokes `ffmpeg` (see [FFmpeg Integration](#ffmpeg-integration) below) to perform the MOV to MP4 transcoding.
        7.  Returns a JSON response (`ConversionResult`) indicating success (with original and output filenames) or failure (with an error message).
    *   `GET /download/{filename}`: Allows downloading of the converted MP4 file. It serves the specified file from the `output/` directory using `actix_files::NamedFile`.
    *   `GET /static/*`: Serves static files (CSS, JS) from the `./static` directory.
    *   `GET /output/*`: Serves files directly from the `./output` directory. (Note: `show_files_listing()` is enabled, allowing browsing of this directory.)

3.  **File Handling:**
    *   Uploaded files are streamed to disk to handle potentially large files efficiently.
    *   UUIDs ensure that concurrent conversions or re-uploads of same-named files do not cause conflicts in the `output/` directory.

### FFmpeg Integration

The conversion relies on executing `ffmpeg` as an external command-line process.

*   **Probing Command:**
    ```bash
    ffmpeg -i <input_mov_path>
    ```
    The `stderr` from this command is analyzed for early detection of file issues.

*   **Conversion Command:**
    ```bash
    ffmpeg -i <input_mov_path> -vcodec h264 -acodec aac -strict -2 -f mp4 -y <output_mp4_path>
    ```
    *   `-i <input_mov_path>`: Specifies the input MOV file.
    *   `-vcodec h264`: Sets the video codec to H.264 (libx264 typically, one of the most compatible video codecs).
    *   `-acodec aac`: Sets the audio codec to AAC (Advanced Audio Coding).
    *   `-strict -2` (or `-strict experimental` in older FFmpeg versions): Often used to enable the native FFmpeg AAC encoder. Modern FFmpeg versions might not strictly require this for AAC, but `-2` here is used for more lenient processing.
    *   `-f mp4`: Forces the output container format to MP4.
    *   `-y`: Overwrites the output file if it already exists. This is a safeguard, though UUIDs should prevent name clashes.

## Error Handling

The application implements error handling at multiple levels:

*   **Client-Side:** Basic validation for `.mov` file type. Network errors during fetch requests are caught.
*   **Backend:**
    *   Handles I/O errors during file saving.
    *   Catches errors from `ffmpeg` execution.
    *   The `get_user_friendly_error` function attempts to parse `ffmpeg`'s `stderr` output to provide more understandable messages for common problems:
        *   Corrupted input files.
        *   Files without valid video/audio streams.
        *   Issues with FFmpeg installation or accessibility.
    *   Returns appropriate HTTP status codes (e.g., `400 Bad Request`, `500 Internal Server Error`) with a JSON payload containing error details.

## Potential Enhancements & Future Work

This tool provides a solid foundation. Here are some potential areas for future development:

*   **Real-time Conversion Progress:** Implement WebSockets or Server-Sent Events (SSE) to parse FFmpeg's progress output and stream it to the client for an accurate progress bar.
*   **Job Queue System:** For improved scalability and handling of multiple concurrent, long-running conversions, integrate a job queue (e.g., using Redis with a Rust worker, or a crate like `fang`).
*   **Advanced FFmpeg Options:** Expose UI controls for users to specify common FFmpeg parameters like resolution, bitrate, CRF (Constant Rate Factor), video filters, etc.
*   **Automated File Cleanup:** Implement a scheduled task (e.g., using a cron job or an in-app scheduler) to delete old files from `uploads/` and `output/` directories to manage disk space.
*   **Containerization:** Provide a `Dockerfile` for easy deployment using Docker, ensuring all dependencies (including FFmpeg) are bundled.
*   **Configuration Management:** Externalize settings like server port, FFmpeg path, and temporary directory paths into a configuration file (e.g., TOML) or environment variables.
*   **Enhanced Input Validation:** More robust server-side validation of uploaded files (e.g., magic byte checking, stricter MIME type validation, file size limits).
*   **Direct S3/Cloud Storage Upload:** Allow users to upload files from or save converted files directly to cloud storage services.
*   **User Authentication:** If deployed in a multi-user environment, add user accounts and authentication.
*   **API for Programmatic Access:** Expose the conversion functionality via a well-defined API for integration with other services.

## A Note on Security

*   **File Uploads:** The application saves uploaded files directly. For a production system exposed to the internet, implement stricter file size limits, content type validation (beyond just extension), and potentially virus scanning.
*   **FFmpeg Security:** Ensure FFmpeg is kept up-to-date to mitigate vulnerabilities associated with media file parsing. Running FFmpeg in a sandboxed environment could be considered for enhanced security.
*   **Directory Listing:** The `/output` endpoint currently has `show_files_listing()` enabled. This means anyone with the URL can browse all converted files. For many use-cases, this might be undesirable. Consider disabling it or implementing access controls if privacy is a concern:
    ```rust
    // In main.rs, change:
    // .service(Files::new("/output", "./output").show_files_listing())
    // to:
    // .service(Files::new("/output", "./output"))
    ```
*   **Resource Exhaustion:** Without proper limits, large files or numerous requests could exhaust server resources (CPU, disk, memory). Implementing rate limiting and file size restrictions is crucial for public-facing deployments.

---

This Mov-To-Mp4-Tool aims to be a practical and performant solution for a common video conversion task, leveraging the strengths of Rust and the ubiquitous FFmpeg.
