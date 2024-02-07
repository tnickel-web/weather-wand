# WeatherWand

![Project Image](docs/images/demo.png)

> A Rust-based command-line tool for fetching weather data from an API and
> presenting it in a beautiful and readable format.

### Note

_This is very much work in progress and the first project in my journey of
learning Rust. It probably does not follow best-practice and still has a lot of
things that can be improved in the future. When looking at this repository,
please take that into consideration._

_I only provide limited support for Windows. Linux is supported. MacOS is not
supported because I do not own a copy of MacOS I can test on. (I hope "it just
works™")._

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)

---

## Introduction

WeatherWand is a command-line tool built in Rust that allows you to retrieve
weather data from an external API, deserialize the JSON response, and present it
in a user-friendly and aesthetically pleasing format in the terminal.

This project was developed with the goal of providing a simple and elegant way
to access weather information for any location and display it with clarity.

---

## Features

- Fetch geolocation & weather data from an API.
- Display weather information in the terminal.
- Support for various units (temperature, wind speed, 12h/24h clock).
- Automated install script for Linux systems.

---

## Requirements

- Set a [Nerd Font](https://github.com/ryanoasis/nerd-fonts) as your terminal
  font to correctly display the icons. The font used in the demo image is
  [JetBrainsMono Nerd Font](https://github.com/ryanoasis/nerd-fonts/tree/master/patched-fonts/JetBrainsMono).
  Other fonts may display the icons differently.
- Ensure a stable internet connection.

---

## Installation

Linux:

There are 2 ways to install this program.

1. Download the latest release manually and place it in a directory of your
   choosing (preferably somewhere in your $PATH).
2. Run the automated installation script in the bin/ directory to install the
   program on your system. The script requires elevated privileges. **As always,
   please check the content of any script for safety before running it.**

```shell
sudo bin/install-linux.sh
```

Windows:

- Download the latest release manually and place it in a directory of your
  choosing.

---

## Usage

The executable is supposed to be run from the terminal. Double-clicking the
executable might not work, since it is a command-line application.

```shell
weather-wand --city <CITY> --temperature-unit <TEMPERATURE_UNIT> --windspeed-unit <WINDSPEED_UNIT> --display <CLOCK_FORMAT>
```

Example:

```shell
weather-wand -c "New York" -t fahrenheit -w mph -d 12h
```

Help:

```shell
weather-wand --help
```

---

## Development

### Requirements

- [rust-clippy](https://github.com/rust-lang/rust-clippy) is used for linting.
  Please run it before every commit:

```shell
cargo clippy
```

- [rustfmt](https://github.com/rust-lang/rustfmt) is used for code formatting.
  Please run it before every commit:

```shell
cargo fmt
```

- _(optional)_ [cargo-modules](https://github.com/regexident/cargo-modules) can
  be used to display the module structure:

```shell
cargo modules generate tree
```

or

```shell
cargo modules generate tree --types
```

### Contribute

1. Clone the project repository:

```shell
git clone https://github.com/tnickel-web/weather-wand.git
```

2. Navigate to the project directory:

```shell
cd weather-wand
```

3. Run the project:

```shell
cargo run
```

4. Build for release:

- Linux:

```shell
cargo build --release
```

- Windows:

```shell
rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-pc-windows-gnu
```

---

