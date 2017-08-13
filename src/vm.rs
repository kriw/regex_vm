use std::collections::VecDeque;
use std::str::Chars;
use parser::Ast;
use vm_ir::Ir;

enum State {
    Run,
    Fail,
    Hlt,
}

pub fn is_match(s: String, mut ir: Vec<Ir>) -> bool {
    let mut q = VecDeque::new();
    q.push_back((0, s.chars()));
    while !q.is_empty() {
        if let Some((head, mut chs)) = q.pop_front() {
            if ir.len() <= head {
                continue;
            }
            let rs = exec(ir[head as usize], &mut chs);
            for r in rs {
                match r.1 {
                    State::Run => q.push_back((r.0 as usize, chs.clone())),
                    State::Fail => {},
                    State::Hlt => {
                        //HACKME
                        if chs.clone().peekable().peek().is_none() {
                            return true;
                        }
                    },
                }
            }
        }
    }
    return false;
}

fn exec(code: Ir, head: &mut Chars) -> Vec<(u32, State)> {
    match code {
        Ir::Char(pc, ch) => {
            let nxt = head.next();
            let st = if nxt.is_none() {
                State::Fail
            }else if ch == nxt.unwrap() {
                State::Run
            }else {
                State::Fail
            };
            vec![(pc + 1, st)]
        },
        Ir::Jmp(pc, pc_to) => vec![(pc_to, State::Run)],
        Ir::Split(pc, pc_to1, pc_to2) => vec![(pc_to1, State::Run), (pc_to2, State::Run)],
        Ir::Nop(pc) => vec![(pc + 1, State::Run)],
        Ir::Hlt(pc) => {
            vec![(pc, State::Hlt)]
        },
    }
}
