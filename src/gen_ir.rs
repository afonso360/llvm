/// Beware, this is the most experimental macro you have ever
/// used, you are absolutley on your own here, it may or may
/// not eat your program alive if you use it.
///
/// This macro attempts to generate the calls for the instructions
/// that are in its body, the generated IR may not be exactly the
/// same as what you are expecting, if you would like to directly
/// insert raw IR you can use the `insert_ir` function of the
/// `Builder` module
// This will eventually become a compiler plugin in order to
// support the full IR syntax, but for now this is enough
#[unstable]
macro_rules! gen_ir {
    ($builder: expr, { $(%$ret: ident = $inst: tt $(%$args: expr),+)*}) => { interpolate_idents! {
        $(
            let $ret = $builder.[build_ $inst]($($args),+, stringify!($ret));
        )*
    }}
}

#[cfg(test)]
mod tests {
    use std::mem;
    use module::Module;
    use context::Context;
    use std::fmt::Display;
    use execution_engine::ExecutionEngine;
    use target::initialize_native_target;
    use target::initialize_native_asm_printer;
    use types::{function_type, ContextType};
    use builder::Builder;

    fn run_function_in_module<A: Display>(m: Module) -> String{
        initialize_native_target();
        initialize_native_asm_printer();
        let ee = ExecutionEngine::create_for_module(&m).unwrap();
        let addr = ee.get_function_address("fname").unwrap();

        unsafe {
            let f: extern "C" fn() -> A = mem::transmute(addr);
            format!("{}", f())
        }
    }

    fn build_env() -> (Context, Module, Builder) {
        let context = Context::new();
        let mut module = context.module_create_with_name("sum");
        let mut builder = context.create_builder();

        let function_type = function_type(
            i64::get_type_in_context(&context), vec![], false);
        let mut func = module.add_function(function_type, "fname");
        let bb = context.append_basic_block(&mut func, "fname");
        builder.position_at_end(bb);
        (context, module, builder)
    }

    #[test]
    fn test_add() {
        let (context, module, mut builder) = build_env();

        let b = context.cons(10i8);
        let c = context.cons(10i8);

        gen_ir! (builder, {
            %l = add %b, %c
        });
        builder.build_ret(l);

        assert_eq!(run_function_in_module::<i8>(module), "20");
    }
}
