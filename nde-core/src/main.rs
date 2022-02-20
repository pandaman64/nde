use std::{cell::RefCell, collections::HashMap};

use rnix::{
    types::{BinOpKind, ParsedType, Root, TypedNode, Wrapper},
    value::Anchor,
    SyntaxNode,
};
use typed_arena::Arena;

// TODO: metadata (positions, ...)
#[derive(Debug, Clone)]
enum Term<'arena> {
    // normal forms
    Error,
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    SimpleString(String),
    SimplePath(Anchor, String),
    // weak-head normal forms
    List(Vec<Thunk<'arena>>),
    AttrSet(HashMap<String, Thunk<'arena>>),
    InterpolatedString(),
    InterpolatedPath(),

    // other terms
    Apply {
        function: Thunk<'arena>,
        argument: Thunk<'arena>,
    },
    Assert {
        predicate: Thunk<'arena>,
        result: Thunk<'arena>,
    },
    BinOp {
        operator: BinOpKind,
        lhs: Thunk<'arena>,
        rhs: Thunk<'arena>,
    },
    Condition {
        condition: Thunk<'arena>,
        then_result: Thunk<'arena>,
        else_result: Thunk<'arena>,
    },
    Function(),
    LetIn(),
    With {
        attr_set: Thunk<'arena>,
        result: Thunk<'arena>,
    },
    Wrapper(Thunk<'arena>),
}

impl<'arena> Term<'arena> {
    fn as_value(&'arena self) -> Option<&'arena Term<'arena>> {
        match self {
            Term::Error
            | Term::Null
            | Term::Boolean(_)
            | Term::Integer(_)
            | Term::Float(_)
            | Term::SimpleString(_)
            | Term::SimplePath(_, _) => Some(self),
            _ => None,
        }
    }

    fn is_value(&'arena self) -> bool {
        self.as_value().is_some()
    }
}

#[derive(Debug, Clone)]
struct Thunk<'arena> {
    origin: ParsedType,
    versions: RefCell<Vec<&'arena Term<'arena>>>,
}

impl<'arena> Thunk<'arena> {
    fn new(origin: SyntaxNode) -> Self {
        Self {
            origin: ParsedType::cast(origin).unwrap(),
            versions: RefCell::new(vec![]),
        }
    }

    fn as_value(&self) -> Option<&'arena Term<'arena>> {
        self.versions.borrow().last()?.as_value()
    }

    fn is_value(&self) -> bool {
        self.as_value().is_some()
    }

    // TODO: allow stepping every subterm
    fn step(&self, arena: &'arena Arena<Term<'arena>>) -> Option<&'arena Term<'arena>> {
        if let Some(value) = self.as_value() {
            return Some(value);
        }
        let next_version = match self.versions.borrow().last() {
            None => {
                let term = match &self.origin {
                    ParsedType::Root(root) => Term::Wrapper(Thunk::new(root.inner().unwrap())),
                    ParsedType::Paren(paren) => Term::Wrapper(Thunk::new(paren.inner().unwrap())),
                    ParsedType::Value(value) => match value.to_value().unwrap() {
                        rnix::NixValue::Float(float) => Term::Float(float),
                        rnix::NixValue::Integer(integer) => Term::Integer(integer),
                        rnix::NixValue::String(string) => Term::SimpleString(string),
                        rnix::NixValue::Path(anchor, string) => Term::SimplePath(anchor, string),
                    },
                    ParsedType::Apply(_) => todo!(),
                    ParsedType::Assert(_) => todo!(),
                    ParsedType::Key(_) => todo!(),
                    ParsedType::Dynamic(_) => todo!(),
                    ParsedType::Error(_) => todo!(),
                    ParsedType::Ident(_) => todo!(),
                    ParsedType::IfElse(_) => todo!(),
                    ParsedType::Select(_) => todo!(),
                    ParsedType::Inherit(_) => todo!(),
                    ParsedType::InheritFrom(_) => todo!(),
                    ParsedType::Lambda(_) => todo!(),
                    ParsedType::LegacyLet(_) => todo!(),
                    ParsedType::LetIn(_) => todo!(),
                    ParsedType::List(list) => Term::List(list.items().map(Thunk::new).collect()),
                    ParsedType::BinOp(binop) => Term::BinOp {
                        operator: binop.operator().unwrap(),
                        lhs: Thunk::new(binop.lhs().unwrap()),
                        rhs: Thunk::new(binop.rhs().unwrap()),
                    },
                    ParsedType::OrDefault(_) => todo!(),
                    ParsedType::PatBind(_) => todo!(),
                    ParsedType::PatEntry(_) => todo!(),
                    ParsedType::Pattern(_) => todo!(),
                    ParsedType::AttrSet(_) => todo!(),
                    ParsedType::KeyValue(_) => todo!(),
                    ParsedType::Str(_) => todo!(),
                    ParsedType::UnaryOp(_) => todo!(),
                    ParsedType::With(_) => todo!(),
                    ParsedType::PathWithInterpol(_) => todo!(),
                };
                Some(&*arena.alloc(term))
            }
            Some(term) => match term {
                Term::Error
                | Term::Null
                | Term::Boolean(_)
                | Term::Integer(_)
                | Term::Float(_)
                | Term::SimpleString(_)
                | Term::SimplePath(_, _) => unreachable!(),
                Term::Wrapper(wrapper) => wrapper.step(arena),
                Term::List(_) => todo!(),
                Term::AttrSet(_) => todo!(),
                Term::InterpolatedString() => todo!(),
                Term::InterpolatedPath() => todo!(),
                Term::Apply { function, argument } => todo!(),
                Term::BinOp { operator, lhs, rhs } => todo!(),
                Term::Assert { predicate, result } => match predicate.step(arena) {
                    Some(Term::Boolean(true)) => result.step(arena),
                    Some(Term::Boolean(false)) => Some(&*arena.alloc(Term::Error)),
                    Some(term) if term.is_value() => Some(&*arena.alloc(Term::Error)),
                    Some(_) => None,
                    None => None,
                },
                Term::Condition {
                    condition,
                    then_result,
                    else_result,
                } => todo!(),
                Term::Function() => todo!(),
                Term::LetIn() => todo!(),
                Term::With { attr_set, result } => todo!(),
            },
        };
        if let Some(next_version) = next_version {
            self.versions.borrow_mut().push(next_version);
        }
        self.as_value()
    }
}

fn main() {
    let arena = Arena::new();
    let source = std::fs::read_to_string("corpus/integer.nix").unwrap();
    let ast = rnix::parse(&source);

    let thunk = Thunk::new(ast.node());
    thunk.step(&arena);
    println!("{} {:?}", thunk.is_value(), thunk);
    thunk.step(&arena);
    println!("{} {:?}", thunk.is_value(), thunk);
    thunk.step(&arena);
    println!("{} {:?}", thunk.is_value(), thunk);
    thunk.step(&arena);
    println!("{} {:?}", thunk.is_value(), thunk);
    thunk.step(&arena);
    println!("{} {:?}", thunk.is_value(), thunk);
    thunk.step(&arena);
    println!("{} {:?}", thunk.is_value(), thunk);
}
