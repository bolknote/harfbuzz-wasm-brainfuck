use harfbuzz_wasm::{Font, Glyph, GlyphBuffer};
use wasm_bindgen::prelude::*;

fn interpret_brainfuck(program: &str, input: &str) -> String {
    const TAPE_SIZE: usize = 30_000;
    let mut tape = vec![0u8; TAPE_SIZE];
    let mut ptr = 0;
    let mut loop_stack = Vec::new();
    let mut pc = 0;
    let program_bytes = program.as_bytes();
    let mut output = String::new();
    let mut input_chars = input.chars();
    let mut current_input = || input_chars.next().unwrap_or('\0');

    while pc < program_bytes.len() {
        match program_bytes[pc] as char {
            '>' => ptr = (ptr + 1) % TAPE_SIZE,
            '<' => ptr = (ptr + TAPE_SIZE - 1) % TAPE_SIZE,
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '.' => output.push(tape[ptr] as char),
            ',' => tape[ptr] = current_input() as u8,
            '[' => {
                if tape[ptr] == 0 {
                    let mut loop_level = 1;
                    while loop_level > 0 {
                        pc += 1;
                        loop_level += match program_bytes[pc] as char {
                            '[' => 1,
                            ']' => -1,
                            _ => 0,
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            },
            ']' => {
                if tape[ptr] != 0 {
                    pc = *loop_stack.last().unwrap();
                } else {
                    loop_stack.pop();
                }
            },
            _ => {},
        }
        pc += 1;
    }

    output
}

#[wasm_bindgen]
pub fn shape(
    _shape_plan: u32,
    font_ref: u32,
    buf_ref: u32,
    _features: u32,
    _num_features: u32,
) -> i32 {
    let font = Font::from_ref(font_ref);
    let mut buffer = GlyphBuffer::from_ref(buf_ref);
    
    let buf_u8: Vec<u8> = buffer.glyphs.iter().map(|g| g.codepoint as u8).collect();
    let str_buf = String::from_utf8_lossy(&buf_u8);

    let mut parts = str_buf.splitn(2, '/');
    let program = parts.next().unwrap_or("");
    let input = parts.next().unwrap_or("");
    
    let interpreted_output = interpret_brainfuck(program, input);
    
    buffer.glyphs = create_glyphs_from_string(&interpreted_output);
    
    for glyph in &mut buffer.glyphs {
        glyph.codepoint = font.get_glyph(glyph.codepoint, 0);
        glyph.x_advance = font.get_glyph_h_advance(glyph.codepoint);
    }
    
    1
}

fn create_glyphs_from_string(input_str: &str) -> Vec<Glyph> {
    input_str
        .chars()
        .filter(|&c| c != '\n') // Удаляем символы новой строки
        .enumerate()
        .map(|(ix, c)| Glyph {
            codepoint: c as u32,
            flags: 0,
            x_advance: 0,
            y_advance: 0,
            cluster: ix as u32,
            x_offset: 0,
            y_offset: 0,
        })
        .collect()
}
