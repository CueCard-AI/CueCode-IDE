---
title: Install CueCode - macOS, Linux, Windows
description: Download and install CueCode on macOS, Linux, or Windows. Includes direct download and package manager options.
---

# Installing CueCode

## Download CueCode

### macOS

Get the latest stable builds via [the download page](https://cuecode.dev/download). If you want to download our preview build, you can find it on its [releases page](https://cuecode.dev/releases/preview). After the first manual installation, CueCode will periodically check for install updates.

### Windows

Get the latest stable builds via [the download page](https://cuecode.dev/download). If you want to download our preview build, you can find it on its [releases page](https://cuecode.dev/releases/preview). After the first manual installation, CueCode will periodically check for install updates.

Additionally, you can install CueCode using winget when a CueCode package is published for your channel. Until then, use the download page above.

### Linux

For most Linux users, the easiest way to install CueCode is through our installation script:

```sh
curl -f https://cuecode.dev/install.sh | sh
```

You can now optionally specify a **version** of CueCode to install using the `ZED_VERSION` environment variable:

```sh
# Install the latest stable version (default)
curl -f https://cuecode.dev/install.sh | sh

# Install a specific version
curl -f https://cuecode.dev/install.sh | ZED_VERSION=0.216.0 sh
```

To install the preview build, which receives updates about a week ahead of stable:

```sh
curl -f https://cuecode.dev/install.sh | ZED_CHANNEL=preview sh
```

This script supports `x86_64` and `AArch64`, as well as common Linux distributions: Ubuntu, Arch, Debian, RedHat, CentOS, Fedora, and more.

If CueCode is installed using this installation script, it can be uninstalled at any time by running the shell command `cuecode --uninstall`. The shell will then prompt you whether you'd like to keep your preferences or delete them. After making a choice, you should see a message that CueCode was successfully uninstalled.

If this script is insufficient for your use case, you run into problems running CueCode, or there are errors in uninstalling CueCode, please see our [Linux-specific documentation](./linux.md).

## System Requirements

### macOS

CueCode supports the following macOS releases:

| Version       | Codename | Apple Status   | CueCode Status          |
| ------------- | -------- | -------------- | ------------------- |
| macOS 26.x    | Tahoe    | Supported      | Supported           |
| macOS 15.x    | Sequoia  | Supported      | Supported           |
| macOS 14.x    | Sonoma   | Supported      | Supported           |
| macOS 13.x    | Ventura  | Supported      | Supported           |
| macOS 12.x    | Monterey | EOL 2024-09-16 | Supported           |
| macOS 11.x    | Big Sur  | EOL 2023-09-26 | Partially Supported |
| macOS 10.15.x | Catalina | EOL 2022-09-12 | Partially Supported |

The macOS releases labelled "Partially Supported" (Big Sur and Catalina) do not support screen sharing via CueCode Collaboration. These features use the [LiveKit SDK](https://livekit.io) which relies upon [ScreenCaptureKit.framework](https://developer.apple.com/documentation/screencapturekit/) only available on macOS 12 (Monterey) and newer.

#### Mac Hardware

CueCode supports machines with Intel (x86_64) or Apple (aarch64) processors that meet the above macOS requirements:

- MacBook Pro (Early 2015 and newer)
- MacBook Air (Early 2015 and newer)
- MacBook (Early 2016 and newer)
- Mac Mini (Late 2014 and newer)
- Mac Pro (Late 2013 or newer)
- iMac (Late 2015 and newer)
- iMac Pro (all models)
- Mac Studio (all models)

### Linux

CueCode supports 64-bit Intel/AMD (x86_64) and 64-bit Arm (aarch64) processors.

CueCode requires a Vulkan 1.3 driver and the following desktop portals:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`
- `org.freedesktop.portal.Secret` or `org.freedesktop.Secrets`

### Windows

CueCode supports the following Windows releases:
| Version | CueCode Status |
| ------------------------- | ------------------- |
| Windows 11, version 22H2 and later | Supported |
| Windows 10, version 1903 and later | Supported |

A 64-bit operating system is required to run CueCode.

#### Windows Hardware

CueCode supports machines with x64 (Intel, AMD) or Arm64 (Qualcomm) processors that meet the following requirements:

- Graphics: A GPU that supports DirectX 11 (most PCs from 2012+).
- Driver: Current NVIDIA/AMD/Intel/Qualcomm driver (not the Microsoft Basic Display Adapter).

### FreeBSD

Not yet available as an official download. Can be built [from source](./development/freebsd.md).

### Web

Not supported at this time. See our [Platform Support issue](https://github.com/zed-industries/zed/issues/5391).
