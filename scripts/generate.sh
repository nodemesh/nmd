#!/bin/sh
protoc --rust_out ./src ./src/messages.proto
