use variant_enum::typed_variant;

#[typed_variant(derive(Debug))]
#[derive(derive_more::From)]
pub enum Msg {
    #[inner(derive(Clone))]
    A {
        pub a: u32,
        b: f32,
        c: usize,
        d: String,
    },
    B {
        foo: String,
    },
    C {
        bar: bool,
    },
}

// generated:
// #[derive(Debug)]
// #[derive(derive_more::From)]
// pub enum WorkerMsg {
//     A(A),
//     B(B),
//     C(C),
// }
// #[derive(Clone)]
// #[derive(Debug)]
// pub struct A {
//     pub a: u32,
//     b: f32,
//     c: usize,
//     d: String,
// }
// #[derive(Debug)]
// pub struct B {
//     foo: String,
// }
// #[derive(Debug)]
// pub struct C {
//     bar: bool,
// }
// impl TryFrom<WorkerMsg> for A {
//     type Error = WorkerMsg;
//     fn try_from(value: WorkerMsg) -> Result<Self, Self::Error> { if let WorkerMsg::A(m) = value { Ok(m) } else { Err(value) } }
// }
// impl TryFrom<WorkerMsg> for B {
//     type Error = WorkerMsg;
//     fn try_from(value: WorkerMsg) -> Result<Self, Self::Error> { if let WorkerMsg::B(m) = value { Ok(m) } else { Err(value) } }
// }
// impl TryFrom<WorkerMsg> for C {
//     type Error = WorkerMsg;
//     fn try_from(value: WorkerMsg) -> Result<Self, Self::Error> { if let WorkerMsg::C(m) = value { Ok(m) } else { Err(value) } }
// }

fn main() {}
