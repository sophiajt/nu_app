use std::{
    io::BufReader,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

use nu_engine::{eval_block, eval_block_with_early_return};
use nu_parser::parse;
use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    print_if_stream, BufferedReader, CliError, PipelineData, RawStream, Span, Value,
};

pub fn set_last_exit_code(stack: &mut Stack, exit_code: i64) {
    stack.add_env_var(
        "LAST_EXIT_CODE".to_string(),
        Value::int(exit_code, Span::unknown()),
    );
}

pub fn report_error_new(
    engine_state: &EngineState,
    error: &(dyn miette::Diagnostic + Send + Sync + 'static),
) {
    let working_set = StateWorkingSet::new(engine_state);

    report_error(&working_set, error);
}

pub fn report_error(
    working_set: &StateWorkingSet,
    error: &(dyn miette::Diagnostic + Send + Sync + 'static),
) {
    eprintln!("Error: {:?}", CliError(error, working_set));
    // reset vt processing, aka ansi because illbehaved externals can break it
    #[cfg(windows)]
    {
        let _ = nu_utils::enable_vt_processing();
    }
}

pub fn get_init_cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| {
        std::env::var("PWD")
            .map(Into::into)
            .unwrap_or_else(|_| nu_path::home_dir().unwrap_or_default())
    })
}

pub fn eval_source(
    engine_state: &mut EngineState,
    stack: &mut Stack,
    source: &[u8],
    fname: &str,
    input: PipelineData,
    allow_return: bool,
) -> bool {
    let (block, delta) = {
        let mut working_set = StateWorkingSet::new(engine_state);
        let output = parse(
            &mut working_set,
            Some(fname), // format!("entry #{}", entry_num)
            source,
            false,
        );
        if let Some(err) = working_set.parse_errors.first() {
            set_last_exit_code(stack, 1);
            report_error(&working_set, err);
            return false;
        }

        (output, working_set.render())
    };

    if let Err(err) = engine_state.merge_delta(delta) {
        set_last_exit_code(stack, 1);
        report_error_new(engine_state, &err);
        return false;
    }

    let b = if allow_return {
        eval_block_with_early_return(engine_state, stack, &block, input, false, false)
    } else {
        eval_block(engine_state, stack, &block, input, false, false)
    };

    match b {
        Ok(pipeline_data) => {
            let result;
            if let PipelineData::ExternalStream {
                stdout: stream,
                stderr: stderr_stream,
                exit_code,
                ..
            } = pipeline_data
            {
                result = print_if_stream(stream, stderr_stream, false, exit_code);
            } else {
                result = pipeline_data.print(engine_state, stack, true, false);
            }

            match result {
                Err(err) => {
                    let working_set = StateWorkingSet::new(engine_state);

                    report_error(&working_set, &err);

                    return false;
                }
                Ok(exit_code) => {
                    set_last_exit_code(stack, exit_code);
                }
            }

            // reset vt processing, aka ansi because illbehaved externals can break it
            #[cfg(windows)]
            {
                let _ = enable_vt_processing();
            }
        }
        Err(err) => {
            set_last_exit_code(stack, 1);

            let working_set = StateWorkingSet::new(engine_state);

            report_error(&working_set, &err);

            return false;
        }
    }
    true
}

pub fn create_stdin_input() -> PipelineData {
    // stdin
    let stdin = std::io::stdin();
    let buf_reader = BufReader::new(stdin);

    // ctrl-c
    let ctrlc = Arc::new(AtomicBool::new(false));

    PipelineData::ExternalStream {
        stdout: Some(RawStream::new(
            Box::new(BufferedReader::new(buf_reader)),
            Some(ctrlc),
            Span::unknown(),
            None,
        )),
        stderr: None,
        exit_code: None,
        span: Span::unknown(),
        metadata: None,
        trim_end_newline: false,
    }
}

pub fn create_engine_state() -> EngineState {
    crate::create_default_context::create_default_context()
}

pub fn create_stack() -> nu_protocol::engine::Stack {
    // stack
    let mut stack = nu_protocol::engine::Stack::new();

    let path = get_init_cwd();

    stack.add_env_var(
        "PWD".into(),
        Value::String {
            val: path.to_string_lossy().to_string(),
            internal_span: Span::unknown(),
        },
    );

    stack
}
