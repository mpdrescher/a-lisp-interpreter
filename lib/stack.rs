use scope::Scope;
use error::Error;
use value::Value;

//TODO: DO IT
#[derive(Debug)]
pub struct Stack {
    scopes: Vec<Scope>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            scopes: Vec::new()
        }
    }

    pub fn from_scopes(scopes: Vec<Scope>) -> Stack {
        Stack {
            scopes: scopes
        }
    }

    pub fn size(&self) -> usize {
        self.scopes.len()
    }

    pub fn pop(&mut self) {
        let _ = self.scopes.pop();
    }

    pub fn push(&mut self, scope: Scope) {
        self.scopes.push(scope);
    }

    pub fn into_first_scope(self) -> Option<Scope> {
        self.scopes.into_iter().next()
    }

    pub fn get_mut_first(&mut self) -> Option<&mut Scope> {
        self.scopes.get_mut(0)
    }

    pub fn resolve_variable(&self, var: &String) -> Result<Value, Error> {
        if self.size() == 0 {
            return Err(Error::new(format!("tried to resolve variable on empty stack.")));
        }
        let mut counter = self.size() - 1;
        loop {
            let scope = self.scopes.get(counter).unwrap();
            if scope.has_variable(&var) {
                return Ok(scope.get_variable(&var).unwrap().clone());
            }
            if counter == 0 {
                break;
            }
            counter -= 1;
        }
        Err(Error::new(format!("unknown variable '{}'.", var)))
    }

    pub fn set_or_append_variable(&mut self, var: String, value: Value) -> Result<(), Error> {
        if self.size() == 0 {
            return Err(Error::new(format!("tried to set variable on empty stack.")));
        }
        let mut counter = self.size() - 1;
        loop {
            let scope = self.scopes.get_mut(counter).unwrap();
            if scope.has_variable(&var) {
                scope.set_variable(var, value);
                return Ok(());
            }
            if counter == 0 {
                break;
            }
            counter -= 1;
        }
        if self.size() == 1 {
            return Err(Error::new(format!("no scope above the current one.")));
        }
        let index = self.size() - 2;
        self.scopes.get_mut(index).unwrap().set_variable(var, value);
        Ok(())
    }
}