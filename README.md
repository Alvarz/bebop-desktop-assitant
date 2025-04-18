# Gemini Desktop assistant in Rust

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) <!-- Replace with your actual license -->
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/<your_crate_name>.svg)](https://crates.io/crates/<your_crate_name>) <!-- Replace with your crate name, if published -->

This app enables real-time AI interaction within any text field. By clicking the "bb" trigger, the app starts capturing your typed text. Pressing the Enter key sends the buffered input to an AI engine for processing. The AI-generated response is then returned and displayed within the same text area, allowing for a fluid and integrated AI-assisted writing experience.

**Features:**

*   **Real-time Text Capture:** Continuously monitors and captures user input.
*   **"bb" Activation:** Starts buffering text input upon clicking the "bb" button/trigger.
*   **Buffered Input:** Temporarily stores typed text after "bb" activation.
*   **AI Processing Trigger:** Sends the buffered text to an AI engine upon pressing the Enter key.
*   **Contextual AI Response:**  Receives and displays the AI-generated response directly within the existing text area.​
*   **Integrated Workflow:** Creates a seamless and conversational AI-assisted experience.
*   **Dynamic AI Interaction:** Allows for immediate and responsive interaction with AI.​


**Caveats**

Experience dynamic AI interaction on your Mac! This application captures your text input, and after clicking the "bb" button, it buffers your words. Pressing Enter sends the buffered text to the AI for processing, with the response seamlessly integrated. Important: This app requires administrative privileges for proper operation and is only compatible with macOS.

## Prerequisites

*   **Rust Installation:**  Ensure you have Rust installed. You can download it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
*   **Gemini API Key:**  You need a Google Gemini API key.  You can obtain one from [https://makersuite.google.com/app/apikey](https://makersuite.google.com/app/apikey).  (This link might change; refer to official Google AI documentation).

## Installation

1.  **Clone the repository:**​
​
    ```bash
    git clone https://github.com/Alvarz/bebop-desktop-assistant
    cd bebop-desktop-assistant
    ```

2.  **Build the application:**
​
    ```bash
    cargo build --release
    ```

    This creates an executable in the `target/release` directory.

3.  **(Optional) Install the application (for system-wide access):**

    ```bash
    cargo install --path .
    ```
    This command will install the executable to your cargo bin directory.  Make sure this directory is in your system's PATH environment variable. You can find this directory by executing `cargo env | grep CARGO_HOME` and appending `/bin` to the result.

## Configuration

It's highly recommended to configure your Gemini API key. You can do this in a few ways:

*   **Environment Variable:**  Set the `GEMINI_API_KEY` environment variable. This is the most secure way to store your API key.

    ```bash
    export GEMINI_API_KEY="YOUR_API_KEY"  # Linux/macOS
    set GEMINI_API_KEY=YOUR_API_KEY      # Windows
    ```

*   **Configuration File (Recommended):** Create a configuration file (e.g., `config.toml` or `.env`) to store the API key and other settings.  This keeps your code clean.  Here's an example `config.toml`:

    ```toml
    # config.toml

    api_key = "YOUR_API_KEY"
    # You can add other settings here, such as model parameters.
    ```

    You'll need to use a crate like `config` or `dotenv` in your Rust code to load this file.  See the "Usage" section for how to integrate this.
​
*   **Command-Line Argument (Least Secure):**  You *could* pass the API key as a command-line argument, but this is generally discouraged because it might be visible in your shell history.


## Building a Chat History

If you implement a `chat` command, you'll likely want to persist the conversation history.  Here are a few ideas:

*   **In-Memory:** Store the history in a `Vec<String>` within your program.  This is simple but lost when the program exits.

*   **File-Based:** Save the history to a file (e.g., `chat_history.txt`).  This is more persistent.  Consider using a format like JSON or YAML for structured storage.​
​
*   **Database:** For more advanced applications, consider using a database (e.g., SQLite, PostgreSQL) to store the chat history.

## Error Handling

Your application should handle potential errors gracefully, such as:​
​
*   **Invalid API Key:** Check if the API key is valid.
*   **Network Errors:** Handle network connection issues.
*   **Gemini API Errors:**  Parse and display error messages from the Gemini API.
*   **Invalid Input:**  Validate user input.

## Contributing
​
Contributions are welcome! Please follow these steps:

1.  Fork the repository.​
2.  Create a new branch for your feature or bug fix.
3.  Make your changes and commit them with clear, descriptive messages.
4.  Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.  (Update this with your actual license file and information.)

## Acknowledgments

*   The Rust language and its awesome community.
*   Google for providing the Gemini (Bard) API.
​