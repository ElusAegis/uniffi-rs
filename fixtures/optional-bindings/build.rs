/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

fn main() {
    if cfg!(feature = "optional") {
        println!("cargo::rustc-env=UNIFFI_CARGO_BUILD_EXTRA_ARGS=--features=optional");
    }
}
