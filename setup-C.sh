#!/bin/bash

GREEN="\033[1;32m"
YELLOW="\033[1;33m"
BLUE="\033[1;34m"
RESET="\033[0m"

echo -e "${GREEN}Installing clang...${RESET}"

sudo apt-get update
sudo apt-get install -y clang clang-tidy

echo -e "${GREEN}Installing cpplint...${RESET}"

sudo apt-get install -y cpplint
