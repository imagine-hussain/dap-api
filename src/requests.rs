use crate::ProtocolMessage;

trait RequestProtocolMessage: ProtocolMessage {
    type Args;

    fn get_command(&self) -> String;
    fn get_arguments(&self) -> Option<Self::Args>;
}

pub enum Request {
    /// The attach request is sent from the client to the debug adapter to
    /// attach to a debuggee that is already running.
    /// 
    /// Since attaching is debugger/runtime specific, the arguments for this
    /// request are not part of this specification.
    // TODO: NEEDS TO do a `extend_args` or something
    Attach,
    BreakpointLocations,
    Completions,
    ConfigurationDone,
    Continue,
    DataBreakpointInfo,
    Disassemble,
    Disconnect,
    Evaluate,
    ExceptionInfo,
    Goto,
    GotoTargets,
    Initialize,
    Launch,
    LoadedSources,
    Modules,
    Next,
    Pause,
    ReadMemory,
    Restart,
    RestartFrame,
    ReverseContinue,
    Scopes,
    SetBreakpoints,
    SetDataBreakpoints,
    SetExceptionBreakpoints,
    SetExpression,
    SetFunctionBreakpoints,
    SetInstructionBreakpoints,
    SetVariable,
    Source,
    StackTrace,
    StepBack,
    StepIn,
    StepInTargets,
    StepOut,
    Terminate,
    TerminateThreads,
    Threads,
    Variables,
    WriteMemory,
}