use std::fmt::{Debug, Formatter};

pub(crate) trait Stack<T>
where
    T:Debug,
{
    fn push(&mut self,item:T);
    fn pop(&mut self,)->T;
    fn len(&self) -> usize;
    fn display(&self,f: &mut Formatter<'_>);
}

impl<T: std::fmt::Display + std::fmt::Debug> Stack<T> for Vec<T> {

    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn pop(&mut self) -> T {
        self.pop().unwrap()
    }

    fn len(&self) -> usize{
        self.len()
    }

    fn display(&self,f: &mut Formatter<'_>){
        write!(f, "{:?}", self).unwrap();
    }

}

impl<T: std::fmt::Debug> Debug for dyn Stack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.display(f);

        return Ok(());
    }
}

