use lisp::DataItem::Int;
use lisp::run;
use lisp::Token::Data;

macro_rules! result_of {
    ($name:ident, $a:expr => $b:expr) => {
        #[test] fn $name() {
            assert_eq!(run($a).unwrap(), $b);
        }
    }
}

result_of! { one_plus_one, "1 + 1"             => Data(Int(2))  }
result_of! { plus_one_one, "+ 1 1"             => Data(Int(2))  }
result_of! { one_one_plus, "1 1 +"             => Data(Int(2))  }
result_of! { many_add,     "5 8 1 0 + + +"     => Data(Int(14)) }
result_of! { paren_add,    "(1 + 1) (6 + 2) +" => Data(Int(10)) }