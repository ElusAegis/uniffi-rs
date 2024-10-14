# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import asyncio
import unittest
from uniffi_optional_bindings import *

class TestOptionalBindings(unittest.TestCase):
    def test_default_add(self):
        add(1, 2)

    def test_optional_add(self):
         optional_add(1, 2)

if __name__=='__main__':
    unittest.main()
