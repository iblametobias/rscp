#!/usr/bin/env bash
# usage: ./check.sh [time_limit_s=1] [mem_limit_mb=256]

cd "$(dirname "$0")/.." || exit 1

TL=${1:-1}
ML=${2:-256}

[ -n "$CP_NO_BUILD" ] || cargo build -q || exit 1
rustc --edition 2021 -O send.rs -o target/send 2>/dev/null || { echo "send.rs failed to compile"; exit 1; }

pass=0; fail=0
for in in tests/*.in; do
  exp="${in%.in}.out"
  [ -f "$exp" ] || continue
  out=$(mktemp); meta=$(mktemp)

  /usr/bin/time -f "%e %M" -o "$meta" timeout "$TL" ./target/send < "$in" > "$out" 2>/dev/null
  code=$?
  read -r t kb < <(tail -n 1 "$meta")
  mb=$(awk -v k="${kb:-0}" 'BEGIN{printf "%.1f", k/1024}')

  verdict=ok
  if   [ $code -eq 124 ]; then verdict="TLE"
  elif [ $code -ne 0 ];   then verdict="RE (exit $code)"
  elif awk -v m="$mb" -v l="$ML" 'BEGIN{exit !(m>l)}'; then verdict="MLE"
  elif ! diff -wB -q "$out" "$exp" >/dev/null; then verdict="WA"
  fi

  if [ "$verdict" = ok ]; then
    printf "ok    %-20s %ss  %sMB\n" "$in" "$t" "$mb"; pass=$((pass+1))
  else
    printf "FAIL  %-20s %s  (%ss  %sMB)\n" "$in" "$verdict" "$t" "$mb"
    [ "$verdict" = WA ] && diff -wB "$out" "$exp" | head -n 10
    fail=$((fail+1))
  fi
  rm -f "$out" "$meta"
done
echo "$pass passed, $fail failed"
[ $fail -eq 0 ]
