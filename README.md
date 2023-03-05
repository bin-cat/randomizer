# Randomizer

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/)
- [pnpm](https://pnpm.io/) (or Yarn/npm/whatever)
- [BASS audio library](https://www.un4seen.com/), add-ons for it (BASSALAC, BASSFLAC, BASSWMA, BASS_AAC are expected by default)
- [Microsoft WebView2 Runtime](https://go.microsoft.com/fwlink/p/?LinkId=2124703)
- [NSIS](https://sourceforge.net/projects/nsis/) (optional)

## Extra directory

Contains files that are not needed in building process but for running and creating installer (third-party dynamic libraries, sampledata files).

```text
extra/
├── data/                            (sample lists, background and sounds, optional)
├── plugins/                         (BASS plugins)
├── bass.dll                         (BASS library itself)
└── MicrosoftEdgeWebview2Setup.exe   (Microsoft WebView2 Runtime installer)
```

## Building

Run `pnpm tb` in project root

## Development

Run `pnpm td` in project root
