pub enum Code {
    PUSH {
        val: i32
    },
    PLUS,
    MULT
}

pub fn new_push(i : i32) -> Code {
    return Code::PUSH {val:i}
}

pub fn new_plus() -> Code {
    return Code::PLUS
}

pub fn new_mult() -> Code {
    return Code::MULT
}

pub struct VM {
    code: Vec<Code>,
    stack: Vec<i32>
}

impl VM {
    pub fn run(&mut self) -> Option<i32> {
        self.stack = Vec::from([]);

        for c in &self.code{
            match c {
                Code::PUSH {val} => self.stack.push(*val),
                Code::PLUS => {
                    let left = self.stack.pop();
                    let right = self.stack.pop();
                    if (left.is_none() || right.is_none()) {
                        self.stack.clear();
                        return None;
                    }
                    self.stack.push(left.unwrap() + right.unwrap());
                }
                Code::MULT => {
                    let left = self.stack.pop();
                    let right = self.stack.pop();
                    if (left.is_none() || right.is_none()) {
                        self.stack.clear();
                        return None;
                    }
                    self.stack.push(left.unwrap() * right.unwrap());
                }
            }
        }
        return self.stack.pop()
    }
}

pub fn setup_vm(code: Vec<Code>) -> VM{
    return VM{code, stack: Vec::from([])}
}

#[cfg(test)]
mod tests {
    use crate::vm::{new_mult, new_plus, new_push, setup_vm};

    #[test]
    fn test_vm_1(){
        let vec = vec![new_push(1),new_push(2),new_push(3),new_mult(),new_plus()];

        let mut vm = setup_vm(vec);
        let ret = vm.run();
        assert_eq!(7, ret.unwrap())
    }

    #[test]
    fn test_vm_2(){
        let vec = vec![new_push(2),new_push(4),new_push(3),new_plus(),new_plus()];

        let mut vm = setup_vm(vec);
        let ret = vm.run();
        assert_eq!(9, ret.unwrap())
    }

    #[test]
    fn test_vm_3(){
        let vec = vec![new_push(1),new_push(2),new_push(3),new_push(4),new_mult(),new_mult(),new_mult()];

        let mut vm = setup_vm(vec);
        let ret = vm.run();
        assert_eq!(24, ret.unwrap())
    }

    #[test]
    fn test_vm_4(){
        let vec = vec![new_push(1),new_push(2),new_push(3),new_mult(),new_plus(),new_plus(),new_plus()];

        let mut vm = setup_vm(vec);
        let ret = vm.run();
        assert_eq!(true,ret.is_none())
    }
}