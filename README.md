# hyprnome

[GNOME](https://www.gnome.org/)-like workspace switching in [Hyprland](https://hyprland.org/).

## Features

- Switch between workspaces without worrying about which numbered workspace you're on.
- Reduce the amount of effort it takes to switch workspaces since you no longer have to reach for keys 3-9.
- **Difference between `m+1`**: Does not wrap and instead creates new workspaces when reaching the end.
- **Difference between `r+1`**: Does not show an empty workspace if there's a non-empty workspace with a higher id.

## Installation

### [NixOS](https://nixos.wiki/wiki/Overview_of_the_NixOS_Linux_distribution) (Recommended)

Add [`hyprnome`](https://search.nixos.org/packages?channel=unstable&query=hyprnome) to your `systemPackages` and rebuild.

```nix
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    hyprnome
  ];
}
```

Alternatively, use `nix run nixpkgs#hyprnome` to try hyprnome without installing it.

### [Arch Linux](https://archlinux.org/)

```fish
git clone https://github.com/donovanglover/hyprnome && cd hyprnome && makepkg -si
```

### Other distributions

Follow the [install guide](https://www.rust-lang.org/tools/install) for Rust. Then, use cargo to install hyprnome.

```fish
cargo install --git https://github.com/donovanglover/hyprnome --tag 0.1.0
```

## Usage

```man
Usage: hyprnome [OPTIONS]

Options:
  -p, --previous         Go to the previous workspace instead of the next
  -n, --no-empty-before  Don't create empty workspaces to the left
  -N, --no-empty-after   Don't create empty workspaces to the right
  -m, --move             Move the active window to the dispatched workspace
  -v, --verbose          Print debugging information
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

Example `hyprland.conf`:

```bash
bind = $SUPER, 1, exec, hyprnome --previous
bind = $SUPER, 2, exec, hyprnome
bind = $SUPER_SHIFT, 1, exec, hyprnome --previous --move
bind = $SUPER_SHIFT, 2, exec, hyprnome --move
```

## Contributing

This software should be bug-free, however contributions are welcome. Remember to write tests for any new functionality and ensure that all existing tests pass.
