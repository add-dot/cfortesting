[![pipeline status](https://gitlab.com/var0G/cft/badges/master/pipeline.svg)](https://gitlab.com/var0G/cft/-/commits/master)
## cftesting
A command-line interface tool written in Rust to easily download Chrome for testing,
Chromedriver, Chrome Headless Shell binaries.

### Why this project?
To get "chromium" for testing, just to avoid to install npm and other
dependencies while exploring different versions of "chromium" for testing and
drivers to interact with you tools of choice. so this CLI can help to get 
test running on the versions you install you just need to specifie browser and
drivers versions as need.

### Platform Support
Currently we Only support **Linux**.
Windows and macOs, need to be tested if you come across this and want to try it
and share results contact me or create an issue to review and work on it.

#### Examples:
Install the latest stable version of Chrome:
```bash
cfortesting install chrome
```
# or
```bash
cfortesting install chrome@stable
```
Install a specific version of ChromeDriver:
```bash
cfortesting install chromedriver@116.0.5845.96
```
Install the Canary channel of Chrome Headless Shell:
```
cfortesting install chrome-headless-shell@canary
```

#### Where are the binaries stored?
All downloaded and extracted binaries are stored in your home directory under the .cft hidden folder, structured by binary type and version.
Example path on Linux/macOS:
```
~/.cft/chromedriver/116.0.5845.96/
~/.cft/chrome-headless-shell/154.0.8025.0/
~/.cft/chrome/154.0.8025.0
```

#### License
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
This project is licensed under the MIT License. See the [LICENSE](https://gitlab.com/var0G/cft/-/blob/master/LICENSE) file for details.
