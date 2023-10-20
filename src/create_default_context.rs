use nu_cmd_lang::*;
use nu_command::*;
use nu_protocol::engine::{EngineState, StateWorkingSet};

pub fn create_default_context() -> EngineState {
    let mut engine_state = EngineState::new();

    let delta = {
        let mut working_set = StateWorkingSet::new(&engine_state);

        macro_rules! bind_command {
            ( $( $command:expr ),* $(,)? ) => {
                $( working_set.add_decl(Box::new($command)); )*
            };
        }

        // Core
        bind_command! {
            Alias,
            Break,
            Collect,
            Const,
            Continue,
            Def,
            DefEnv,
            Describe,
            Do,
            Echo,
            ErrorMake,
            ExportAlias,
            ExportCommand,
            ExportConst,
            ExportDef,
            ExportDefEnv,
            ExportExtern,
            ExportExternWrapped,
            ExportUse,
            ExportModule,
            Extern,
            ExternWrapped,
            For,
            Hide,
            HideEnv,
            If,
            Ignore,
            Overlay,
            OverlayUse,
            OverlayList,
            OverlayNew,
            OverlayHide,
            LazyMake,
            Let,
            Loop,
            Match,
            Module,
            Mut,
            Return,
            Scope,
            ScopeAliases,
            ScopeCommands,
            ScopeEngineStats,
            ScopeExterns,
            ScopeModules,
            ScopeVariables,
            Try,
            Use,
            Version,
            While,
        };

        #[cfg(feature = "plugin")]
        bind_command!(Register);

        // Charts
        bind_command! {
            Histogram
        }

        // Filters
        bind_command! {
            All,
            Any,
            Append,
            Columns,
            Compact,
            Default,
            Drop,
            DropColumn,
            DropNth,
            Each,
            Empty,
            Enumerate,
            Every,
            Filter,
            Find,
            First,
            Flatten,
            Get,
            Group,
            GroupBy,
            Headers,
            Insert,
            Items,
            Join,
            SplitBy,
            Take,
            Merge,
            Move,
            TakeWhile,
            TakeUntil,
            Last,
            Length,
            Lines,
            ParEach,
            Prepend,
            Range,
            Reduce,
            Reject,
            Rename,
            Reverse,
            Select,
            Shuffle,
            Skip,
            SkipUntil,
            SkipWhile,
            Sort,
            SortBy,
            SplitList,
            Transpose,
            Uniq,
            UniqBy,
            Upsert,
            Update,
            Values,
            Where,
            Window,
            Wrap,
            Zip,
        };

        // Misc
        bind_command! {
            Source,
            Tutor,
        };

        // Path
        bind_command! {
            Path,
            PathBasename,
            PathDirname,
            PathExists,
            PathExpand,
            PathJoin,
            PathParse,
            PathRelativeTo,
            PathSplit,
            PathType,
        };

        // System
        bind_command! {
            Complete,
            External,
            NuCheck,
            Sys,
        };

        // Help
        bind_command! {
            Help,
            HelpAliases,
            HelpExterns,
            HelpCommands,
            HelpModules,
            HelpOperators,
            HelpEscapes,
        };

        // Debug
        bind_command! {
            Ast,
            Debug,
            DebugInfo,
            Explain,
            Inspect,
            Metadata,
            Profile,
            TimeIt,
            View,
            ViewFiles,
            ViewSource,
            ViewSpan,
        };

        #[cfg(unix)]
        bind_command! { Exec }

        #[cfg(windows)]
        bind_command! { RegistryQuery }

        #[cfg(any(
            target_os = "android",
            target_os = "linux",
            target_os = "macos",
            target_os = "windows"
        ))]
        bind_command! { Ps };

        #[cfg(feature = "which-support")]
        bind_command! { Which };

        // Strings
        bind_command! {
            Char,
            Decode,
            Encode,
            DecodeBase64,
            EncodeBase64,
            DetectColumns,
            Parse,
            Size,
            Split,
            SplitChars,
            SplitColumn,
            SplitRow,
            SplitWords,
            Str,
            StrCapitalize,
            StrContains,
            StrDistance,
            StrDowncase,
            StrEndswith,
            StrExpand,
            StrJoin,
            StrReplace,
            StrIndexOf,
            StrLength,
            StrReverse,
            StrStartsWith,
            StrSubstring,
            StrTrim,
            StrUpcase,
            FormatDate,
            FormatDuration,
            FormatFilesize,
        };

        // FileSystem
        bind_command! {
            Cd,
            Ls,
            Mkdir,
            Mv,
            Cp,
            UCp,
            Open,
            Start,
            Rm,
            Save,
            Touch,
            Glob,
            Watch,
        };

        // Platform
        bind_command! {
            Ansi,
            AnsiStrip,
            Clear,
            Du,
            Input,
            InputList,
            InputListen,
            Kill,
            Sleep,
            TermSize,
        };

        // Date
        bind_command! {
            Date,
            DateHumanize,
            DateListTimezones,
            DateNow,
            DateToRecord,
            DateToTable,
            DateToTimezone,
        };

        // Shells
        bind_command! {
            Exit,
        };

        // Formats
        bind_command! {
            From,
            FromCsv,
            FromJson,
            FromNuon,
            FromOds,
            FromSsv,
            FromToml,
            FromTsv,
            FromXlsx,
            FromXml,
            FromYaml,
            FromYml,
            To,
            ToCsv,
            ToJson,
            ToMd,
            ToNuon,
            ToText,
            ToToml,
            ToTsv,
            Touch,
            Upsert,
            Where,
            ToXml,
            ToYaml,
        };

        // Viewers
        bind_command! {
            Griddle,
            Table,
        };

        // Conversions
        bind_command! {
            Fill,
            Into,
            IntoBool,
            IntoBinary,
            IntoDatetime,
            IntoDuration,
            IntoFloat,
            IntoFilesize,
            IntoInt,
            IntoRecord,
            IntoString,
            IntoValue,
        };

        // Env
        bind_command! {
            ExportEnv,
            LoadEnv,
            SourceEnv,
            WithEnv,
            ConfigNu,
            ConfigEnv,
            ConfigMeta,
            ConfigReset,
        };

        // Math
        bind_command! {
            Math,
            MathAbs,
            MathAvg,
            MathCeil,
            MathFloor,
            MathMax,
            MathMedian,
            MathMin,
            MathMode,
            MathProduct,
            MathRound,
            MathSqrt,
            MathStddev,
            MathSum,
            MathVariance,
            MathLog,
        };

        // Bytes
        bind_command! {
            Bytes,
            BytesLen,
            BytesStartsWith,
            BytesEndsWith,
            BytesReverse,
            BytesReplace,
            BytesAdd,
            BytesAt,
            BytesIndexOf,
            BytesCollect,
            BytesRemove,
            BytesBuild
        }

        // Network
        bind_command! {
            Http,
            HttpDelete,
            HttpGet,
            HttpHead,
            HttpPatch,
            HttpPost,
            HttpPut,
            HttpOptions,
            Url,
            UrlBuildQuery,
            UrlDecode,
            UrlEncode,
            UrlJoin,
            UrlParse,
            Port,
        }

        // Random
        bind_command! {
            Random,
            RandomBool,
            RandomChars,
            RandomDice,
            RandomFloat,
            RandomInt,
            RandomInteger,
            RandomUuid,
        };

        // Generators
        bind_command! {
            Cal,
            Seq,
            SeqDate,
            SeqChar,
            Unfold,
        };

        // Hash
        bind_command! {
            Hash,
            HashMd5::default(),
            HashSha256::default(),
        };

        // Experimental
        bind_command! {
            IsAdmin,
        };

        // Removed
        bind_command! {
            LetEnv,
            DateFormat,
        };

        working_set.render()
    };

    if let Err(err) = engine_state.merge_delta(delta) {
        eprintln!("Error creating default context: {err:?}");
    }

    engine_state
}
