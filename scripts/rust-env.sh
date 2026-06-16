#!/usr/bin/env bash

# yes lldb is godlike, so we need to setup debugging manually
# bow before this you unworthy worm (lldb knows better)
# or in this case we have to teach it a lesson meaning the locations
# that rust finds automatically ;)

SYSROOT=$(rustc --print sysroot)
HOST=$(rustc -vV | grep host | cut -d' ' -f2)
TOOLCHAIN_LIBS="$SYSROOT/lib/rustlib/$HOST/lib"

export LD_LIBRARY_PATH="$PWD/target/debug:$PWD/target/debug/deps:$TOOLCHAIN_LIBS"
echo "set LD_LIBRARY_PATH"
echo $LD_LIBRARY_PATH

if [ -f .env.debug ]; then
  echo "Replacing .env.debug content"
  echo "LD_LIBRARY_PATH=\"$LD_LIBRARY_PATH\""
  cat > .env.debug <<EOF
LD_LIBRARY_PATH=\"$LD_LIBRARY_PATH\"
SF4E_ASSET_PATH="../../assets"
EOF
fi
