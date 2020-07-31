#![allow(non_snake_case)]

pub enum OpError {
    DivByZero,
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl Op {
    pub fn all() -> Vec<Op> {
        vec!(Op::Add, Op::Sub, Op::Mul, Op::Div)
    }
    pub fn apply(&self, left: i64, right: i64) -> Result<i64, OpError> {
        match self {
            Op::Add => Ok(left + right),
            Op::Sub => Ok(left - right),
            Op::Mul => Ok(left * right),
            Op::Div => {
                if right == 0 {
                    Err(OpError::DivByZero)
                } else {
                    Ok(left / right)
                }
            }
        }
    }

    pub fn show(&self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        }
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Const {
        value: i64,
    },
    Expression {
        left: Box<Expression>,
        op: Op,
        right: Box<Expression>,
    }
}

impl Expression {
    pub fn fromConst(value: i64) -> Expression {
        Expression::Const {
            value
        }
    }

    pub fn new(left: &Expression, op: Op, right: &Expression) -> Expression {
        Expression::Expression {
            left: Box::new(left.clone()),
            op,
            right: Box::new(right.clone())
        }
    }

    pub fn total(&self) -> Result<i64, OpError> {
        match self {
            Expression::Const { value } => {
                Ok(*value)
            },

            Expression::Expression { left, op, right } => {
                let leftValie = left.total()?;
                let rightValue = right.total()?;
                op.apply(leftValie, rightValue)
            }
        }
    }

    pub fn show(self) -> String {
        match self {
            Expression::Const { value } => {
                format!("{}", value)
            },
            Expression::Expression { left, op , right} => {
                format!("( {} {} {} )", left.show(), op.show(), right.show())
            }
        }
    }
}

fn combine(leftExpression: Vec<Expression>, rightExpression: Vec<Expression>) -> Vec<Expression> {
    leftExpression.iter().flat_map(|left| {
        rightExpression.iter().flat_map(move |right| {
            Op::all().into_iter().map(move |op| {
                Expression::new(&left, op.clone(), &right)
            })
        })
    }).collect()
}

fn split(dataIn: &[i64]) -> impl Iterator<Item=(&[i64], &[i64])> {
    (1..dataIn.len()).map(move |index| {
        dataIn.split_at(index)
    })
}

fn allExpression(dataIn: &[i64]) -> Vec<Expression>{
    if dataIn.len() == 1 {
        let first = dataIn[0];
        return vec!(Expression::fromConst(first));
    }

    let mut out = Vec::<Expression>::new();

    for (left, right) in split(dataIn) {
        let leftExpression = allExpression(left);
        let rightExpression = allExpression(right);
        
        let mut combination = combine(leftExpression, rightExpression);
        out.append(&mut combination);
    }

    out
}

fn main() {
    let dataIn: Vec<i64> =  vec![1, 3, 7, 10, 15, 50];
    let expected = 765 as i64;

    let dataInSlice = dataIn.as_slice();
    let all = allExpression(dataInSlice);

    println!("Przeszukuję z: {}", all.len());

    for item in all {
        let total = item.total();

        match total {
            Ok(total) => {
                if total == expected {
                    println!("znalezione wyrazenie => {}", item.show());
                }
            },
            Err(_err) => {
                //println!("Błąd wyliczania => {}", item.show());
            }
        }
    }

    println!("Przeszukano wszystkie kombinacje");
}


// struct RangeIter {
//     data: Vec<(Box<dyn Iterator<Item=Expression>>, Box<dyn Iterator<Item=Expression>>)>,
//     current: u64,
// }

// impl Iterator for RangeIter {
//     type Item = Box<dyn Iterator<Item=Expression>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!();
//     }

//     fn new(dataIn: &[i64])
// }

// fn allCombinations(data: &[i64]) -> Box<dyn Iterator<Item=Expression>> {
//     //od 0, do len-1 , dzielic po dwa przedzialy
//     //todo!();
//     panic!("das");

// }

// fn main() {
//     let dataIn: Vec<i64> =  vec![1, 3, 7, 10, 15, 50];
//     let expected = 765 as i64;

//     let dataInSlice = dataIn.as_slice();

//     for item in allCombinations(dataInSlice) {
//         if item.total() == expected {
//             println!("znalezione wyrazenie => {}", item.show());
//         }
//     }

//     println!("Przeszukano wszystkie kombinacje");
// }

// let results = count_down::solutions(vec![1, 3, 7, 10, 15, 50], 765);
// for result in results {
//     println!("results === {:?}", result);
// }


//iterator [] ---> listę z mozliwymi iteracjami kolejnych kombinacji

//fn convert()

/*
    mamy iterator, ktory dostarcza podzialu wejsciowej tablicy na dwie mniejsze tab lice
    oraz dostarcza kombinacje roznych tablic wejsciowych

    tablica jest zamieniana na iterator, iterator ktory dostarcza kolejne wyrazenia

    dwa iteratory sa laczone na wielszy iterator, to laczenie musi dokonac kombinacji z kazdel wartosci
    zwracanej z pomniejszych iteratorow
*/
