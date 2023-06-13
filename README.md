# Jetsam

Jetsam is a tool designed to sanitize IRC logs stored in the Driftwood format. It helps identify and flag lines in the log files that contain potentially sensitive or inappropriate content for further review or moderation.

## Features

- Parses log files stored in the Driftwood format, separating columns using a unique Unicode character as a field separator.
- Sanitizes log lines by adding a "#" symbol in the first column to flag them for further review or moderation.
- Flexible wordlist usage: Allows the use of single wordlist files or a directory of wordlist files for customizable content sanitization.
- Supports recursive parsing of wordlist files to handle complex and nested directory structures.
- Supports log files with the .txt extension, adhering to the Driftwood format specification.

## Usage

To use Jetsam, run the following command:
```shell
jetsam <log directory path> <wordlist path>
```
- `<log directory path>`: Path to the directory containing the log files in the Driftwood format.
- `<wordlist path>`: Path to the wordlist file or directory for content sanitization.

## Output

Jetsam modifies the log files by adding a "#" symbol in the first column of sanitized lines. This modification flags the lines for further review or moderation.

#### Example Driftwood Entry:
```
☕12☕34☕56☕GitHubFAN23☕Hello, world!☕
```

This line will be modified to:
```
#☕12☕34☕56☕GitHubFAN23☕Hello, world!☕
```

#### Jetsam logfile entry
```
Timestamp: 20230613-143200
Path: /logs/freenode/programming/2003/01/01.txt
Line Number: 2
Original Line: 12 34 56 GitHubFAN23 Hello, world!
```

---

## Considerations

- Input Validation: Ensure that the provided log directory and wordlist paths are valid and exist. Jetsam does not perform extensive input validation, so it's essential to validate the input to avoid errors.
- Backups: Before running Jetsam on your log files, make sure to create backups of your original files. This precaution helps prevent accidental data loss or unintended modifications.
- Data Security: Treat the log files containing potentially sensitive information with care. Take appropriate measures to protect the data, such as restricting access permissions and following security best practices.

---

## Flowchart
```
┌─ Start Program
│
├─ Load Log Directory
│   ├─ Load Wordlist
│   │   ├─ Read Wordlist File
│   │   └─ Recursively Read Wordlist Directory
│   │
│   ├─ Read Log Files
│   │   ├─ Read Log File
│   │   │   └─ Process Log Lines
│   │   │       ├─ Sanitize Line Content
│   │   │       ├─ Check for Wordlist Match
│   │   │       └─ Modify Line Number and Content
│   │   │
│   │   └─ Recursively Read Log Directory
│   │
│   └─ Log Changes to Jetsam Log
│       └─ Create Jetsam Log File
│           └─ Iterate Modified Lines
│               ├─ Get Timestamp
│               ├─ Get Log File Path
│               ├─ Get Line Number
│               ├─ Get Original Line Content
│               └─ Write to Jetsam Log File
│
└─ End Program
```

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

---

## License

These files released under the [MIT License](LICENSE).
