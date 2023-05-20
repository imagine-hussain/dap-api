use crate::{Number, Source};


///
pub struct AttachArgs {
    /// Arbitrary data from the previous, restarted session.
    /// The data is sent as the `restart` attribute of the `terminated` event.
    /// The client should leave the data intact.
    /// 
    /// Should be json (eventually :P)
    __restart: Option<String>,
}

/// The breakpointLocations request returns all possible locations for source
/// breakpoints in a given range.
/// Clients should only call this request if the corresponding capability
/// 
/// supportsBreakpointLocationsRequest is true.
pub struct BreakpointLocationsArgs {
    /// The source location of the breakpoints; either `source.path` or
    /// `source.reference` must be specified.
    source: Source,

    /// Start line of range to search possible breakpoint locations in. If only the
    /// line is specified, the request returns all possible locations in that line.
    line: Option<Number>,

    /// Start position within `line` to search possible breakpoint locations in. It
    /// is measured in UTF-16 code units and the client capability
    /// `columnsStartAt1` determines whether it is 0- or 1-based. If no column is
    /// given, the first position in the start line is assumed.
    column: Option<Number>,

    /// End line of range to search possible breakpoint locations in. If no end
    /// line is given, then the end line is assumed to be the start line.
    endLine: Option<Number>,

    /// End position within `endLine` to search possible breakpoint locations in.
    /// It is measured in UTF-16 code units and the client capability
    /// `columnsStartAt1` determines whether it is 0- or 1-based. If no end column
    /// is given, the last position in the end line is assumed.
    endColumn: Option<Number>,
}

pub struct CompletionsArgs {}

pub struct ConfigurationDoneArgs {}

pub struct ContinueArgs {}

pub struct DataBreakpointInfoArgs {}

pub struct DisassembleArgs {}

pub struct DisconnectArgs {}

pub struct EvaluateArgs {}

pub struct ExceptionInfoArgs {}

pub struct GotoArgs {}

pub struct GotoTargetsArgs {}

pub struct InitializeArgs {}

pub struct LaunchArgs {}

pub struct LoadedSourcesArgs {}

pub struct ModulesArgs {}

pub struct NextArgs {}

pub struct PauseArgs {}

pub struct ReadMemoryArgs {}

pub struct RestartArgs {}

pub struct RestartFrameArgs {}

pub struct ReverseContinueArgs {}

pub struct ScopesArgs {}

pub struct SetBreakpointsArgs {}

pub struct SetDataBreakpointsArgs {}

pub struct SetExceptionBreakpointsArgs {}

pub struct SetExpressionArgs {}

pub struct SetFunctionBreakpointsArgs {}

pub struct SetInstructionBreakpointsArgs {}

pub struct SetVariableArgs {}

pub struct SourceArgs {}

pub struct StackTraceArgs {}

pub struct StepBackArgs {}

pub struct StepInArgs {}

pub struct StepInTargetsArgs {}

pub struct StepOutArgs {}

pub struct TerminateArgs {}

pub struct TerminateThreadsArgs {}

pub struct ThreadsArgs {}

pub struct VariablesArgs {}

pub struct WriteMemoryArgs {}
