// Copyright 2014 The Gfx-rs Developers.
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

#![feature(plugin)]

extern crate gfx;
#[macro_use]
extern crate gfx_macros;

use secret_lib::gfx::ShaderSource;

mod secret_lib;

static _SRC1: ShaderSource<'static> = shaders! {
glsl_120: b"
    #version 120
    attribute vec3 a_Pos;
    attribute vec2 a_TexCoord;
    varying vec2 v_TexCoord;
    uniform mat4 u_ModelViewProj;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_ModelViewProj * vec4(a_Pos, 1.0);
    }
",
glsl_150: b"
    #version 150 core
    in vec3 a_Pos;
    in vec2 a_TexCoord;
    out vec2 v_TexCoord;
    uniform mat4 u_ModelViewProj;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_ModelViewProj * vec4(a_Pos, 1.0);
    }
"
};

static _SRC2: ShaderSource<'static> = shaders! {
glsl_120: b"
    #version 120
    attribute vec3 a_Pos;
    attribute vec2 a_TexCoord;
    varying vec2 v_TexCoord;
    uniform mat4 u_ModelViewProj;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_ModelViewProj * vec4(a_Pos, 1.0);
    }
"
};

static _SRC3: ShaderSource<'static> = shaders! {
glsl_150: b"
    #version 150 core
    in vec3 a_Pos;
    in vec2 a_TexCoord;
    out vec2 v_TexCoord;
    uniform mat4 u_ModelViewProj;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_ModelViewProj * vec4(a_Pos, 1.0);
    }
"
};

static _SRC4: ShaderSource<'static> = shaders! {
targets: &["o_Color", "o_Foo"],
};

static _SRC5: ShaderSource<'static> = shaders! {
};
