use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use std::collections::HashMap;

fn generate_llvm_ir(ast: BFProgram) {
    let context = Context::create();
    let module = context.create_module("brainfuck");
    let builder = context.create_builder();

    let main_function = module.add_function("main", context.i32_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(basic_block);

    let memory = builder.build_alloca(context.i8_type(), "memory");
    let pointer = builder.build_alloca(context.i8_type().ptr_type(inkwell::AddressSpace::Generic), "pointer");
    
    builder.build_store(memory, context.i8_type().const_zero());
    builder.build_store(pointer, context.i8_type().ptr_type(inkwell::AddressSpace::Generic).const_zero());

    generate_code(&context, &builder, &ast.tokens, memory, pointer);

    builder.build_return(Some(&context.i32_type().const_int(0, false)));
}

fn generate_code(
    context: &Context,
    uilder: &Builder,
    tokens: &[BFToken],
    memory: PointerValue,
    pointer: PointerValue,
) {
    let int8_type = context.i8_type();
    let int32_type = context.i32_type();

    for token in tokens {
        match token {
            BFToken::Increment => {
                let current_value = builder.build_load(memory, "current_value");
                let incremented_value = builder.build_int_add(current_value, int8_type.const_int(1, false), "incremented_value");
                builder.build_store(memory, incremented_value);
            }
            BFToken::Decrement => {
                let current_value = builder.build_load(memory, "current_value");
                let decremented_value = builder.build_int_sub(current_value, int8_type.const_int(1, false), "decremented_value");
                builder.build_store(memory, decremented_value);
            }
            BFToken::MoveRight => {
                let current_pointer = builder.build_load(pointer, "current_pointer");
                let incremented_pointer = builder.build_gep(current_pointer, &[int32_type.const_int(1, false)], "incremented_pointer");
                builder.build_store(pointer, incremented_pointer);
            }
            BFToken::MoveLeft => {
                let current_pointer = builder.build_load(pointer, "current_pointer");
                let decremented_pointer = builder.build_gep(current_pointer, &[int32_type.const_int(-1, false)], "decremented_pointer");
                builder.build_store(pointer, decremented_pointer);
            }
            BFToken::Output => {
                let current_value = builder.build_load(memory, "current_value");
                builder.build_call(println_function(context, module), &[current_value], "println");
            }
            BFToken::Input => {
                 generate_input_code(context, builder, memory);
            }
            BFToken::Loop(loop_tokens) => {
                let current_value = builder.build_load(memory, "current_value");
                let is_zero = builder.build_int_compare(inkwell::IntPredicate::EQ, current_value, int8_type.const_int(0, false), "is_zero");

                let function = builder.get_insert_block().get_parent().unwrap();
                let loop_body_block = context.append_basic_block(function, "loop_body");
                let loop_after_block = context.append_basic_block(function, "loop_after");

                builder.build_conditional_branch(is_zero, loop_after_block, loop_body_block);

                builder.position_at_end(loop_body_block);
                generate_code(context, builder, loop_tokens, memory, pointer);

                let current_value = builder.build_load(memory, "current_value");
                let is_zero = builder.build_int_compare(inkwell::IntPredicate::EQ, current_value, int8_type.const_int(0, false), "is_zero");

                builder.build_conditional_branch(is_zero, loop_after_block, loop_body_block);

                builder.position_at_end(loop_after_block);
            }
        }
    }
}

fn generate_input_code(context: &Context, builder: &Builder, memory: PointerValue) {
    let getchar_func = getchar_function(context, module);
    let input_char = builder.build_call(getchar_func, &[], "input_char");

    let input_value = builder.build_int_truncate(
        input_char.try_as_basic_value().unwrap().into_int_value(),
        context.i8_type(),
        "input_value",
    );

    builder.build_store(memory, input_value);
}


fn println_function(context: &Context, module: Module) -> FunctionValue {
    let int32_type = context.i32_type();
    let printf_type = int32_type.fn_type(&[int32_type.into()], true);
    let printf_func = module.add_function("printf", printf_type, None);

    printf_func
}
