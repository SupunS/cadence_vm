#
# Cadence - The resource-oriented smart contract programming language
#
# Copyright 2019-20222 Dapper Labs, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: build-static
build-static:
	@cd runtime && cargo build --release
	@cp runtime/target/release/libruntime.a ./runtime
	go build call_rust.go

.PHONY: run-static
run-static: build-static
	@./main_static

# Run the Rust lib tests natively via cargo
.PHONY: test-rust-lib
test-rust-lib:
	@cd lib/runtime && cargo test -- --nocapture

.PHONY: clean
clean:
	rm -rf main_dynamic main_static libruntime.so libruntime.a runtime/target
