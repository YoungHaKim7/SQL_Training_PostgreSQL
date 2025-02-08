# macOS

```bash
If you need to have libpq first in your PATH, run:
  fish_add_path /opt/homebrew/opt/libpq/bin

For compilers to find libpq you may need to set:
  set -gx LDFLAGS "-L/opt/homebrew/opt/libpq/lib"
  set -gx CPPFLAGS "-I/opt/homebrew/opt/libpq/include"

For pkg-config to find libpq you may need to set:
  set -gx PKG_CONFIG_PATH "/opt/homebrew/opt/libpq/lib/pkgconfig"

```
