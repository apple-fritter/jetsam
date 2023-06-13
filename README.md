# Jetsam

Jetsam is a tool designed to sanitize IRC logs stored in the [Driftwood](https://github.com/apple-fritter/driftwood) format. It helps identify and flag lines in the log files that contain potentially sensitive or inappropriate content for further review or moderation.

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
Original Line: 123456 GitHubFAN23 Hello, world!
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

## 🤪 IRC Meta
### [@apple-fritter](https://github.com/apple-fritter)'s IRC Repositories:

---

#### WeeChat
- [weechat.ban-evasion-detection](https://github.com/apple-fritter/weechat.ban-evasion-detection): Detect and prevent ban evasion. (Python)
- [weechat.typo-aggregator](https://github.com/apple-fritter/weechat.typo-aggregator): Record misspelled words in a TSV (tab-separated values) file. (Python)
- [weechat.whois-aggregator](https://github.com/apple-fritter/weechat.whois-aggregator): Aggregate whois data in a rolling CSV file. (Python)
- [weechat.youtube-info](https://github.com/apple-fritter/weechat.youtube-info): Deprecated. Extract video information from a YouTube URL and post it back to the channel. (Python)
- [weechat.youtube-api](https://github.com/apple-fritter/weechat.youtube-api): Extract video information from a YouTube URL and post it back to the channel. (Python)

---

#### IRCcloud
- [irccloud-to-weechat](https://github.com/apple-fritter/irccloud-to-weechat): Convert IRC logs from IRCcloud format to Weechat format. (Rust)
- [irccloud-to-xchat](https://github.com/apple-fritter/irccloud-to-xchat): Convert IRC logs from IRCcloud format to XChat format. (Rust)

---

#### X-Chat
- [xchat.channel-moderation](https://github.com/apple-fritter/xchat.channel-moderation): Moderate an IRC channel. (Python)
- [doppelganger](https://github.com/apple-fritter/doppelganger): X-Chat mIRC imposter. Fingerprint subversion. (Python bundle)

---

#### Other
- [driftwood](https://github.com/apple-fritter/driftwood): A unified IRC log format definition. (Rust)
- [jetsam](https://github.com/apple-fritter/jetsam): Flag lines of driftwood formatted logs for moderation or further review. (Rust)
- [scrimshaw](https://github.com/apple-fritter/scrimshaw): Create a quoteslist of any given user, from your driftwood formatted logs. (Rust)

---

### IRC usage considerations
When working with any project involving IRC (Internet Relay Chat), it's important to keep the following considerations in mind to ensure a positive and respectful environment for all participants.

#### Philosophy of Use
Tailor your project's behavior and responses to align with the expected norms and conventions of IRC. Take into account the preferences and expectations of IRC users, ensuring that your project provides a seamless and familiar experience within the IRC ecosystem.

#### Foster a Positive and Inclusive Environment
Respect and adhere to the guidelines and policies of the IRC platform you are using. Familiarize yourself with the platform's rules regarding script usage, automation, and acceptable behavior. Comply with the platform's Terms of Service, and be mindful of any limitations or restrictions imposed by the platform. Strive to create an inclusive and welcoming environment where all users can engage respectfully and comfortably.

#### Respect the Rights and Dignity of Other Users
Maintain a polite and courteous demeanor in all interactions. Uphold the fundamental principles of respect, avoiding engagement in illegal, inappropriate, or offensive behavior. This includes refraining from using derogatory or inflammatory language, sharing explicit, triggering, or offensive content, engaging in harassment, or launching personal attacks. Obtain explicit consent before interacting with other users or sending automated responses. Respect the privacy of other users and avoid invading their personal space without their permission.

#### Respect the IRC Community and Channels
Avoid disrupting the normal flow of conversation within IRC channels. Ensure that your project's actions and responses do not cause unnecessary disruptions or inconvenience to other users. Implement mechanisms to prevent spamming or flooding the channel with excessive or irrelevant messages. Handle errors gracefully, preventing unintended behavior or disruptions to the IRC platform or the experiences of other users.

#### Ensure Compatibility
Consider the potential variations in behavior across different IRC platforms and clients. While aiming for compatibility, be aware that certain functionalities may not be available or consistent across all platforms. Test your project on multiple IRC platforms and clients to ensure compatibility and provide the best possible experience for users.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

---

## License

These files released under the [MIT License](LICENSE).
