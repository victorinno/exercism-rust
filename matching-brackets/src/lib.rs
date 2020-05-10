#[derive(Debug)]
struct Stack {
    top: i32,
    items: Vec<char>,
}

impl Stack {
    pub fn push(&mut self, x: &char) {
        self.items.push(x.clone());
        self.top += 1;
    }

    pub fn pop(&mut self) -> char {
        let element: char = match self.items.pop() {
            None => '\0',
            Some(e) => e,
        };
        self.top -= 1;
        return element;
    }

    pub fn is_empty(&mut self) -> bool {
        return self.top == -1;
    }
}

fn is_matching_pair(character1: char, character2: char) -> bool {
    if character1 == '(' && character2 == ')' {
        return true;
    } else if character1 == '{' && character2 == '}' {
        return true;
    } else if character1 == '[' && character2 == ']' {
        return true;
    } else {
        return false;
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Stack = Stack {
        top: -1,
        items: Vec::new(),
    };
    for c in string.chars() {
        if c == '{' || c == '(' || c == '[' {
            stack.push(&c);
        } else if c == '}' || c == ')' || c == ']' {
            if stack.is_empty() {
                return false;
            } else if !is_matching_pair(stack.pop(), c) {
                return false;
            }
        }
    }
    return stack.is_empty();
}
