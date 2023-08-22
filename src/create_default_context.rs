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
            Ast,
            Break,
            Const,
            Continue,
            Debug,
            Def,
            DefEnv,
            Describe,
            Do,
            Echo,
            ErrorMake,
            ExportAlias,
            ExportCommand,
            ExportDef,
            ExportDefEnv,
            ExportExtern,
            ExportUse,
            Extern,
            For,
            Help,
            HelpAliases,
            HelpCommands,
            HelpModules,
            HelpOperators,
            Hide,
            HideEnv,
            If,
            Ignore,
            Overlay,
            OverlayUse,
            OverlayList,
            OverlayNew,
            OverlayHide,
            Let,
            Loop,
            Metadata,
            Module,
            Mut,
            Return,
            Try,
            Use,
            Version,
            While,
        };

        // Charts
        bind_command! {
            Histogram
        }

        // Filters
        bind_command! {
            All,
            Any,
            Append,
            Collect,
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
            StrDowncase,
            StrJoin,
            StrReplace,
            StrIndexOf,
            StrLength,
            StrStartsWith,
            StrSubstring,
            StrTrim,
            StrUpcase
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
            BytesBuild,
        }

        // FileSystem
        bind_command! {
            Cd,
            Cp,
            Ls,
            Mkdir,
            Mv,
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
            Kill,
            Sleep,
            TermSize,
        };

        // Date
        bind_command! {
            Date,
            DateFormat,
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
            Use,
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
            Into,
            IntoBool,
            IntoBinary,
            IntoDatetime,
            IntoDecimal,
            IntoDuration,
            IntoFilesize,
            IntoInt,
            IntoRecord,
            IntoString,
        };

        // Env
        bind_command! {
            ExportEnv,
            LetEnv,
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

        // Network
        bind_command! {
            Http,
            HttpGet,
            HttpPost,
            Url,
            UrlBuildQuery,
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
            RandomDecimal,
            RandomDice,
            RandomInteger,
            RandomUuid,
        };

        // Generators
        bind_command! {
            Cal,
            Seq,
            SeqDate,
            SeqChar,
        };

        // Hash
        bind_command! {
            Hash,
            HashMd5::default(),
            HashSha256::default(),
        };

        // Experimental
        bind_command! {
            ViewSource,
            IsAdmin,
        };

        // Deprecated
        bind_command! {
            Source,
        };

        #[cfg(feature = "plugin")]
        bind_command!(Register);

        working_set.render()
    };

    if let Err(err) = engine_state.merge_delta(delta) {
        eprintln!("Error creating default context: {err:?}");
    }

    engine_state
}
