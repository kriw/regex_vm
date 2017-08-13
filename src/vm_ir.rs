use std::thread;
use parser::Ast;

pub type PC = u32;
#[derive(Debug, Clone, Copy)]
pub enum Ir {
    Char(PC, char),
    Jmp(PC, PC),   //Jmp(p1, p2): jmp to p2 from p1
    Split(PC, PC, PC),
    Nop(PC),
    Hlt(PC),
}

fn top_pc(ir: &Vec<Ir>) -> PC {
    if ir.first().is_none() {
        return 1
    }
    let i = ir.first().unwrap();
    match (*i).clone() {
        Ir::Char(pc, _) => pc,
        Ir::Jmp(pc, _) => pc,
        Ir::Split(pc, _, _) => pc,
        Ir::Nop(pc) => pc,
        Ir::Hlt(pc) => pc,
    }
}


fn compose<F>(ast: Ast, pc: &mut F) -> Vec<Ir>
    where F : FnMut(bool) -> u32 {
    match ast {
        Ast::Char(x)         => {
            vec![Ir::Char(pc(true), x)]
        },
        Ast::Concat(a1, a2)  => {
            let c1 = compose(*a1, pc);
            let c2 = compose(*a2, pc);
            vec![c1, c2].into_iter().flat_map(|x| x).collect::<Vec<_>>()
        },
        Ast::Alt(a1, a2)     => {
            let c0 = pc(true);
            let c1 = compose(*a1, pc);
            let jmp_from = pc(true);
            let c2 = compose(*a2, pc);
            vec![vec![Ir::Split(c0, top_pc(&c1), top_pc(&c2))], c1, vec![Ir::Jmp(jmp_from, pc(false))], c2]
                .into_iter().flat_map(|x| x).collect::<Vec<_>>()
        },
        Ast::Star(a)         => {
            let c1 = compose(*a, pc);
            let c2 = {
                let p = pc(true);
                vec![Ir::Split(p, top_pc(&c1), p+1)]
            };
            c1.into_iter().chain(c2).collect::<Vec<_>>()
        },
    }
}

pub fn compile(ast: Ast) -> Vec<Ir> {
    let mut counter = 0;
    let mut pc = move |inc| {
        if inc {
            let ret = counter;
            counter += 1;
            ret
        } else {
            counter
        }
    };
    let mut code = compose(ast, &mut pc);
    let hlt = vec![Ir::Hlt(pc(true))];
    code.into_iter().chain(hlt).collect::<Vec<_>>()
}

//for debug
pub fn dump(irs: Vec<Ir>) {
    for ir in irs {
        println!("{:?}", ir);
    }
}
