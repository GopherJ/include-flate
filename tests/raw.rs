// include-flate
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(proc_macro_hygiene)]

use include_flate::flate;

flate!(pub static FFFE: [u8] from "assets/009f.dat");

#[test]
fn test() {
    println!("Tests raw");
    assert_eq!(*FFFE, &[0x00u8, 0x9Fu8]);
}
