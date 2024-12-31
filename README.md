# Educational ransomware

A minimalist and tricky program to raise awarness about ransomwares.

## Prerequisites

Make sure you have the following installed on your machine:
- Rustc
- A web browser

## Installation

1. **Clone the repository:**

    ```sh
    git clone git@github.com:Gaspardcode/ransom_educational.git
    cd ransom_educational
    ```

2. **Produce the binary**

    ```sh
    cargo build --release && strip target/release/gui_ransom
    ```
    
## Running the Application

1. **Execute the binary**

    ```sh
    target/release/gui_ransom
    ```

## Important note

The default directory to be encrypted is the trash dir for safety reason.
If you wish to encrypt a another, feel free to modify the global variable ROOT in src/cyp.rs, line 11

I am not responsible for any damage or data loss occured during the use of this software.
