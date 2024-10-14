/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import uniffi.uniffi_optional_bindings.*;

try {
    add(1, 2)
} catch (e: Exception) {
    throw RuntimeException("Should not have failed on add(1, 2)", e)
}

try {
    optionalAdd(1, 2)
} catch (e: Exception) {
    throw RuntimeException("Should not have failed on optionalAdd(1, 2)", e)
}
