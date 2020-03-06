/*
*   Copyright (c) 2020 Adel Prokurov
*   All rights reserved.

*   Licensed under the Apache License, Version 2.0 (the "License");
*   you may not use this file except in compliance with the License.
*   You may obtain a copy of the License at

*   http://www.apache.org/licenses/LICENSE-2.0

*   Unless required by applicable law or agreed to in writing, software
*   distributed under the License is distributed on an "AS IS" BASIS,
*   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*   See the License for the specific language governing permissions and
*   limitations under the License.
*/

pub struct PeepholePass;

use super::*;
use crate::runtime::cell::Function;
use crate::util::arc::Arc;
impl BytecodePass for PeepholePass {
    fn execute(&mut self, f: &mut Arc<Function>) {
        for block in f.code.iter_mut() {
            block.instructions.retain(|x| {
                if let Instruction::Move(to, from) = x {
                    to != from
                } else {
                    true
                }
            });
        }
    }
}
