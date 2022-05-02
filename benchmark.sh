#!/bin/zsh

# Random directory name
RANDOM_TEST_DIR="$(echo $RANDOM | head -c 20)-test"

# Create test directories
mkdir -p ./$RANDOM_TEST_DIR/node_modules
cd ./$RANDOM_TEST_DIR/node_modules || exit
yarn add react react-dom lodash react-icons puppeteer playwright react-scripts webpack prettier eslint babel-cli

mkdir -p ./some/very/nested/node_modules
cd ./some/very/nested/node_modules || exit
yarn add react react-dom lodash react-icons puppeteer playwright react-scripts webpack prettier eslint babel-cli

cd ..

hyperfine --warmup 3 "cargo run -- --dry --path ./$RANDOM_TEST_DIR"

rm -rf "./$RANDOM_TEST_DIR"