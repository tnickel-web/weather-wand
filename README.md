# Weather Fetcher and Beautifier

![Project Image](docs/images/demo.png)

<!-- Include a project image or logo if available -->

> A Rust-based command-line tool for fetching weather data from an API and
> presenting it in a beautiful and readable format.

---

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Requirements](#requirements)
- [Usage](#usage)

---

## Introduction

Weather Fetcher and Beautifier is a command-line tool built in Rust that allows
you to retrieve weather data from an external API, deserialize the JSON
response, and present it in a user-friendly and aesthetically pleasing format in
the terminal.

This project was developed with the goal of providing a simple and elegant way
to access weather information for any location and display it with clarity.

---

## Features

- Fetch weather data from an API.
- Deserialize JSON responses.
- Display weather information in the terminal.
- Support for various units (temperature, wind speed).
- Error handling to ensure reliable operation.

---

## Requirements

- Set a [Nerd Font](https://github.com/ryanoasis/nerd-fonts) as your terminal
  font to correctly display the icons.
- Ensure a stable internet connection.

---

## Usage

```bash
weather-wand --city <CITY> --temperature-unit <TEMPERATURE_UNIT> --windspeed-unit <WINDSPEED_UNIT>
```

Example:

```bash
weather-wand -c "New York" -t fahrenheit -w mph
```

Help:

```bash
weather-wand --help
```
