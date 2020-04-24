#!/bin/bash

# example windows terminal command:
# W:\portables\MrsWatson-0.9.8\Windows\mrswatson64.exe -p W:\dev\rust\rust-dsp-playground\vst-low-pass\target\debug\actondev_dsp_basicvst2.dll -i W:\dev\rust\rust-dsp-playground\vst-low-pass\dev\white_noise_mono.wav -o W:\dev\rust\rust-dsp-playground\vst-low-pass\dev\out.wav

cur_dir=$(dirname "$0")

# converting path to full windows paths (w:\)
winpath="cygpath -w -a"

mrswatchon=$($winpath "/w/portables/MrsWatson-0.9.8/Windows/mrswatson64.exe")
dll=$($winpath $cur_dir/../target/debug/actondev_dsp_basicvst2.dll)
in=$($winpath $cur_dir/white_noise_mono.wav)
out=$($winpath $cur_dir/temp_out.wav)

cmd="$mrswatchon -p $dll -i $in -o $out"

echo running "$cmd"
$cmd