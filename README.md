# Stremio Light

This project uses WRY, libmpv, Vue 3 + TypeScript and is totally early and experimental work.

## Development

### Windows
You will need to download a `libmpv` build from [SourceForge](https://sourceforge.net/projects/mpv-player-windows/files/libmpv/).  
Run this command on the `Developer Command Prompt` to generate a `.lib` file.  

```bash
lib /def:mpv.def /name:mpv-1.dll /out:mpv.lib /MACHINE:X64
```
Then copy the `.dll` and `.lib` files inside the `/lib` folder of the project.

### Debian / Ubuntu

```bash
apt install build-essential libwebkit2gtk-4.0-dev libappindicator3-dev libmpv1 libmpv-dev
```

### Dev

```
npm run serve
cargo run -- --dev
```

### Build

```
npm run build
cargo build
```

