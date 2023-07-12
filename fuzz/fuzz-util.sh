#!/usr/bin/env bash

REPO_DIR=$(git rev-parse --show-toplevel)

listTargetFiles() {
  pushd "$REPO_DIR/fuzz" > /dev/null || exit 1
  find fuzz_targets/ -type f -name "*.rs" | sort
  popd > /dev/null || exit 1
}

targetFileToName() {
  echo "$1" \
    | sed 's/^fuzz_targets\///' \
    | sed 's/\.rs$//' \
    | sed 's/\//_/g'
}

targetFileToHFuzzInputArg() {
  baseName=$(basename "$1")
  dirName="${baseName%.*}"
  if [ -d "hfuzz_input/$dirName" ]; then
    echo "HFUZZ_INPUT_ARGS=\"-f hfuzz_input/$FILE/input\""
  fi
}

listTargetNames() {
  for target in $(listTargetFiles); do
    targetFileToName "$target"
  done
}

# Utility function to avoid CI failures on Windows
checkWindowsFiles() {
  incorrectFilenames=$(find . -type f -name "*,*" -o -name "*:*" -o -name "*<*" -o -name "*>*" -o -name "*|*" -o -name "*\?*" -o -name "*\**" -o -name "*\"*" | wc -l)
  if [ "$incorrectFilenames" -gt 0 ]; then
    echo "Bailing early because there is a Windows-incompatible filename in the tree."
    exit 2
  fi
}

# Checks whether a fuzz case output some report, and dumps it in hex
getReport() {
  reportFile="hfuzz_workspace/$1/HONGGFUZZ.REPORT.TXT"
  if [ -f "$reportFile" ]; then
    cat "$reportFile"
    for CASE in "hfuzz_workspace/$1/SIG"*; do
      xxd -p -c10000 < "$CASE"
    done
    return 1
  fi
  return 0
}

# Check for reports and exit if there are any
checkReport() {
  if ! getReport "$1"; then
    exit 1
  fi
}
