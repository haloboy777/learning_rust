#!/usr/bin/bash

# Script for removing folders after running "cargo new"

dir_list=$(ls -d */)

if [ -z "$1" ]; then
  echo "Error: Provide dir to remove (git/target)"
  exit 1
fi

if [ $1 = "git" ]; then
  type=".git"
elif [ $1 = "target" ]; then
  type="target"
else
  echo "Error: Please provide correct dir to remove (git/target)"
  exit 1
fi

for dir in $dir_list; do

  cd $dir && echo "entering $dir"
  [ -d "$type" ] && rm -rf $type && echo "removed $type"
  cd ..

done
