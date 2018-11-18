extern crate jazz_vm;

use jazz_vm::{function::Function, machine::Machine, opcodes::Instruction, value::Value};

fn main()
{
    let mut machine = Machine::new();
    let string = machine.pool.allocate(Box::new(String::from("disassemble")));

    let code = vec![
        Instruction::LoadInt(2, 2),
        Instruction::Add(2, 1, 2),
        Instruction::Ret(2),
    ];

    let func = Function::from_instructions(code, 1);

    let func = machine.pool.allocate(Box::new(func));

    let code = vec![
        Instruction::LoadObject(2, func),
        Instruction::LoadObject(1, string),
        Instruction::PushArg(2),
        Instruction::LoadAt(2, 2, 1),
        Instruction::PushArg(2),
        Instruction::Call(2, 2, 0),
        Instruction::Ret(2),
    ];

    let func = Value::Object(
        machine
            .pool
            .allocate(Box::new(Function::from_instructions(code, 0))),
    );
    let v = machine.invoke(func, vec![]);
    let obj = if let Value::Object(id) = v {
        machine.pool.get(id)
    } else {
        panic!("");
    };

    println!("{}", obj.to_string(&mut machine));
}
